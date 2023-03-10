// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `runtime_parachains::disputes::slashing`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::disputes::slashing
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/runtime_parachains_disputes_slashing.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::disputes::slashing`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::disputes::slashing::WeightInfo for WeightInfo<T> {
	/// Storage: Session CurrentIndex (r:1 w:0)
	/// Proof Skipped: Session CurrentIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Historical HistoricalSessions (r:1 w:0)
	/// Proof: Historical HistoricalSessions (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: ParaSessionInfo Sessions (r:1 w:0)
	/// Proof Skipped: ParaSessionInfo Sessions (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasSlashing UnappliedSlashes (r:1 w:1)
	/// Proof Skipped: ParasSlashing UnappliedSlashes (max_values: None, max_size: None, mode: Measured)
	/// Storage: Offences ReportsByKindIndex (r:1 w:1)
	/// Proof Skipped: Offences ReportsByKindIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Offences ConcurrentReportsIndex (r:1 w:1)
	/// Proof Skipped: Offences ConcurrentReportsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Offences Reports (r:1 w:1)
	/// Proof Skipped: Offences Reports (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking SlashRewardFraction (r:1 w:0)
	/// Proof: Staking SlashRewardFraction (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ActiveEra (r:1 w:0)
	/// Proof: Staking ActiveEra (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	/// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	/// Proof: Staking ErasStartSessionIndex (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Staking Invulnerables (r:1 w:0)
	/// Proof Skipped: Staking Invulnerables (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking ValidatorSlashInEra (r:1 w:1)
	/// Proof: Staking ValidatorSlashInEra (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking SlashingSpans (r:1 w:1)
	/// Proof Skipped: Staking SlashingSpans (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking SpanSlash (r:1 w:1)
	/// Proof: Staking SpanSlash (max_values: None, max_size: Some(76), added: 2551, mode: MaxEncodedLen)
	/// Storage: Staking OffendingValidators (r:1 w:1)
	/// Proof Skipped: Staking OffendingValidators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking UnappliedSlashes (r:1 w:1)
	/// Proof Skipped: Staking UnappliedSlashes (max_values: None, max_size: None, mode: Measured)
	/// The range of component `n` is `[4, 300]`.
	fn report_dispute_lost(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4631 + n * (209 ±0)`
		//  Estimated: `77841 + n * (2317 ±0)`
		// Minimum execution time: 86_772 nanoseconds.
		Weight::from_parts(124_398_343, 0)
			.saturating_add(Weight::from_parts(0, 77841))
			// Standard Error: 3_624
			.saturating_add(Weight::from_parts(361_232, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().writes(9))
			.saturating_add(Weight::from_parts(0, 2317).saturating_mul(n.into()))
	}
}
