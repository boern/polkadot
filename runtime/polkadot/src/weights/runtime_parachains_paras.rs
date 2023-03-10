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
//! Autogenerated weights for `runtime_parachains::paras`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::paras
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/runtime_parachains_paras.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras::WeightInfo for WeightInfo<T> {
	/// Storage: Paras CurrentCodeHash (r:1 w:1)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PastCodeMeta (r:1 w:1)
	/// Proof Skipped: Paras PastCodeMeta (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PastCodePruning (r:1 w:1)
	/// Proof Skipped: Paras PastCodePruning (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PastCodeHash (r:0 w:1)
	/// Proof Skipped: Paras PastCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:0 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 3145728]`.
	fn force_set_current_code(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8341`
		//  Estimated: `57966`
		// Minimum execution time: 33_964 nanoseconds.
		Weight::from_parts(34_183_000, 0)
			.saturating_add(Weight::from_parts(0, 57966))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(2_374, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// The range of component `s` is `[1, 1048576]`.
	fn force_set_current_head(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_472 nanoseconds.
		Weight::from_parts(8_522_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(972, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Paras Heads (r:0 w:1)
	fn force_set_most_recent_context() -> Weight {
		Weight::from_parts(10_155_000, 0)
			// Standard Error: 0
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeHash (r:1 w:1)
	/// Proof Skipped: Paras FutureCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:1 w:0)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeCooldowns (r:1 w:1)
	/// Proof Skipped: Paras UpgradeCooldowns (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:1 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpcomingUpgrades (r:1 w:1)
	/// Proof Skipped: Paras UpcomingUpgrades (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeUpgrades (r:0 w:1)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeRestrictionSignal (r:0 w:1)
	/// Proof Skipped: Paras UpgradeRestrictionSignal (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 3145728]`.
	fn force_schedule_code_upgrade(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16785`
		//  Estimated: `198990`
		// Minimum execution time: 62_077 nanoseconds.
		Weight::from_parts(63_533_000, 0)
			.saturating_add(Weight::from_parts(0, 198990))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(2_397, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// Proof Skipped: Paras UpgradeGoAheadSignal (max_values: None, max_size: None, mode: Measured)
	/// The range of component `s` is `[1, 1048576]`.
	fn force_note_new_head(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `95`
		//  Estimated: `2760`
		// Minimum execution time: 14_330 nanoseconds.
		Weight::from_parts(14_560_000, 0)
			.saturating_add(Weight::from_parts(0, 2760))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(971, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	fn force_queue_action() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4283`
		//  Estimated: `11536`
		// Minimum execution time: 20_598 nanoseconds.
		Weight::from_parts(21_189_000, 0)
			.saturating_add(Weight::from_parts(0, 11536))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:1 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 3145728]`.
	fn add_trusted_validation_code(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `28`
		//  Estimated: `5006`
		// Minimum execution time: 8_753 nanoseconds.
		Weight::from_parts(8_999_000, 0)
			.saturating_add(Weight::from_parts(0, 5006))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(2_375, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Paras CodeByHashRefs (r:1 w:0)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:0 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	fn poke_unused_validation_code() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `28`
		//  Estimated: `2531`
		// Minimum execution time: 6_338 nanoseconds.
		Weight::from_parts(6_709_000, 0)
			.saturating_add(Weight::from_parts(0, 2531))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `26968`
		//  Estimated: `111832`
		// Minimum execution time: 92_190 nanoseconds.
		Weight::from_parts(94_190_000, 0)
			.saturating_add(Weight::from_parts(0, 111832))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteList (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras UpcomingUpgrades (r:1 w:1)
	/// Proof Skipped: Paras UpcomingUpgrades (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeUpgrades (r:0 w:100)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement_finalize_upgrade_accept() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `27554`
		//  Estimated: `225877`
		// Minimum execution time: 816_058 nanoseconds.
		Weight::from_parts(823_769_000, 0)
			.saturating_add(Weight::from_parts(0, 225877))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(104))
	}
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement_finalize_upgrade_reject() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `27500`
		//  Estimated: `113960`
		// Minimum execution time: 91_497 nanoseconds.
		Weight::from_parts(93_349_000, 0)
			.saturating_add(Weight::from_parts(0, 113960))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteList (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement_finalize_onboarding_accept() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `27022`
		//  Estimated: `169062`
		// Minimum execution time: 643_156 nanoseconds.
		Weight::from_parts(648_775_000, 0)
			.saturating_add(Weight::from_parts(0, 169062))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement_finalize_onboarding_reject() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `26968`
		//  Estimated: `111832`
		// Minimum execution time: 90_809 nanoseconds.
		Weight::from_parts(92_716_000, 0)
			.saturating_add(Weight::from_parts(0, 111832))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
