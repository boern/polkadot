// Copyright 2023 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Memory stats for preparation.
//!
//! Right now we gather three measurements:
//!
//! - `ru_maxrss` (resident set size) from `getrusage`.
//! - `resident` memory stat provided by `tikv-malloc-ctl`.
//! - `allocated` memory stat also from `tikv-malloc-ctl`.
//!
//! Currently we are only logging these for the purposes of gathering data. In the future, we may
//! use these stats to reject PVFs during pre-checking. See
//! <https://github.com/paritytech/polkadot/issues/6472#issuecomment-1381941762> for more
//! background.

use crate::{metrics::Metrics, LOG_TARGET};
use parity_scale_codec::{Decode, Encode};
use std::io;

/// Helper struct to contain all the memory stats, including [`MemoryAllocationStats`] and, if
/// supported by the OS, `ru_maxrss`.
#[derive(Encode, Decode)]
pub struct MemoryStats {
	/// Memory stats from `tikv_jemalloc_ctl`.
	pub memory_tracker_stats: Option<MemoryAllocationStats>,
	/// `ru_maxrss` from `getrusage`. A string error since `io::Error` is not `Encode`able.
	pub max_rss: Option<Result<i64, String>>,
}

/// Statistics of collected memory metrics.
#[non_exhaustive]
#[derive(Clone, Debug, Default, Encode, Decode)]
pub struct MemoryAllocationStats {
	/// Total resident memory, in bytes.
	pub resident: u64,
	/// Total allocated memory, in bytes.
	pub allocated: u64,
}

/// Gets the `ru_maxrss` for the current thread if the OS supports `getrusage`. Otherwise, just
/// returns `None`.
pub fn get_max_rss_thread() -> Option<io::Result<i64>> {
	// `c_long` is either `i32` or `i64` depending on architecture. `i64::from` always works.
	#[cfg(target_os = "linux")]
	let max_rss = Some(getrusage::getrusage_thread().map(|rusage| i64::from(rusage.ru_maxrss)));
	#[cfg(not(target_os = "linux"))]
	let max_rss = None;
	max_rss
}

/// Helper function to send the memory metrics, if available, to prometheus.
pub fn observe_memory_metrics(metrics: &Metrics, memory_stats: MemoryStats, pid: u32) {
	if let Some(max_rss) = memory_stats.max_rss {
		match max_rss {
			Ok(max_rss) => metrics.observe_preparation_max_rss(max_rss as f64),
			Err(err) => gum::warn!(
				target: LOG_TARGET,
				worker_pid = %pid,
				"error getting `ru_maxrss` in preparation thread: {}",
				err
			),
		}
	}

	if let Some(tracker_stats) = memory_stats.memory_tracker_stats {
		// We convert these stats from B to KB to match the unit of `ru_maxrss` from `getrusage`.
		let resident_kb = (tracker_stats.resident / 1024) as f64;
		let allocated_kb = (tracker_stats.allocated / 1024) as f64;

		metrics.observe_preparation_max_resident(resident_kb);
		metrics.observe_preparation_max_allocated(allocated_kb);
	}
}

#[cfg(any(target_os = "linux", feature = "jemalloc-allocator"))]
pub mod memory_tracker {
	use super::*;
	use std::{
		sync::mpsc::{Receiver, RecvTimeoutError, Sender},
		time::Duration,
	};
	use tikv_jemalloc_ctl::{epoch, stats, Error};
	use tokio::task::JoinHandle;

	#[derive(Clone)]
	struct MemoryAllocationTracker {
		epoch: tikv_jemalloc_ctl::epoch_mib,
		allocated: stats::allocated_mib,
		resident: stats::resident_mib,
	}

	impl MemoryAllocationTracker {
		pub fn new() -> Result<Self, Error> {
			Ok(Self {
				epoch: epoch::mib()?,
				allocated: stats::allocated::mib()?,
				resident: stats::resident::mib()?,
			})
		}

		pub fn snapshot(&self) -> Result<MemoryAllocationStats, Error> {
			// update stats by advancing the allocation epoch
			self.epoch.advance()?;

			// Convert to `u64`, as `usize` is not `Encode`able.
			let allocated = self.allocated.read()? as u64;
			let resident = self.resident.read()? as u64;
			Ok(MemoryAllocationStats { allocated, resident })
		}
	}

	/// Runs a thread in the background that observes memory statistics. The goal is to try to get
	/// accurate stats during preparation.
	///
	/// # Algorithm
	///
	/// 1. Create the memory tracker.
	///
	/// 2. Sleep for some short interval. Whenever we wake up, take a snapshot by updating the
	///    allocation epoch.
	///
	/// 3. When we receive a signal that preparation has completed, take one last snapshot and return
	///    the maximum observed values.
	///
	/// # Errors
	///
	/// For simplicity, any errors are returned as a string. As this is not a critical component, errors
	/// are used for informational purposes (logging) only.
	pub fn memory_tracker_loop(finished_rx: Receiver<()>) -> Result<MemoryAllocationStats, String> {
		// This doesn't need to be too fine-grained since preparation currently takes 3-10s or more.
		// Apart from that, there is not really a science to this number.
		const POLL_INTERVAL: Duration = Duration::from_millis(100);

		let tracker = MemoryAllocationTracker::new().map_err(|err| err.to_string())?;
		let mut max_stats = MemoryAllocationStats::default();

		let mut update_stats = || -> Result<(), String> {
			let current_stats = tracker.snapshot().map_err(|err| err.to_string())?;
			if current_stats.resident > max_stats.resident {
				max_stats.resident = current_stats.resident;
			}
			if current_stats.allocated > max_stats.allocated {
				max_stats.allocated = current_stats.allocated;
			}
			Ok(())
		};

		loop {
			// Take a snapshot and update the max stats.
			update_stats()?;

			// Sleep.
			match finished_rx.recv_timeout(POLL_INTERVAL) {
				// Received finish signal.
				Ok(()) => {
					update_stats()?;
					return Ok(max_stats)
				},
				// Timed out, restart loop.
				Err(RecvTimeoutError::Timeout) => continue,
				Err(RecvTimeoutError::Disconnected) =>
					return Err("memory_tracker_loop: finished_rx disconnected".into()),
			}
		}
	}

	/// Helper function to terminate the memory tracker thread and get the stats. Helps isolate all this
	/// error handling.
	pub async fn get_memory_tracker_loop_stats(
		fut: JoinHandle<Result<MemoryAllocationStats, String>>,
		tx: Sender<()>,
	) -> Option<MemoryAllocationStats> {
		// Signal to the memory tracker thread to terminate.
		if let Err(err) = tx.send(()) {
			gum::warn!(
				target: LOG_TARGET,
				worker_pid = %std::process::id(),
				"worker: error sending signal to memory tracker_thread: {}", err
			);
			None
		} else {
			// Join on the thread handle.
			match fut.await {
				Ok(Ok(stats)) => Some(stats),
				Ok(Err(err)) => {
					gum::warn!(
						target: LOG_TARGET,
						worker_pid = %std::process::id(),
						"worker: error occurred in the memory tracker thread: {}", err
					);
					None
				},
				Err(err) => {
					gum::warn!(
						target: LOG_TARGET,
						worker_pid = %std::process::id(),
						"worker: error joining on memory tracker thread: {}", err
					);
					None
				},
			}
		}
	}
}

#[cfg(target_os = "linux")]
mod getrusage {
	use libc::{getrusage, rusage, timeval, RUSAGE_THREAD};
	use std::io;

	/// Get the rusage stats for the current thread.
	pub fn getrusage_thread() -> io::Result<rusage> {
		let mut result = rusage {
			ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
			ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
			ru_maxrss: 0,
			ru_ixrss: 0,
			ru_idrss: 0,
			ru_isrss: 0,
			ru_minflt: 0,
			ru_majflt: 0,
			ru_nswap: 0,
			ru_inblock: 0,
			ru_oublock: 0,
			ru_msgsnd: 0,
			ru_msgrcv: 0,
			ru_nsignals: 0,
			ru_nvcsw: 0,
			ru_nivcsw: 0,
		};
		if unsafe { getrusage(RUSAGE_THREAD, &mut result) } == -1 {
			return Err(io::Error::last_os_error())
		}
		Ok(result)
	}
}