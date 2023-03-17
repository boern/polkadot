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
//! DATE: 2023-03-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
		//  Measured:  `8309`
		//  Estimated: `61734`
		// Minimum execution time: 33_514_000 picoseconds.
		Weight::from_parts(33_804_000, 0)
			.saturating_add(Weight::from_parts(0, 61734))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_953, 0).saturating_mul(c.into()))
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
		// Minimum execution time: 8_575_000 picoseconds.
		Weight::from_parts(8_757_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(866, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().writes(1))
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
		//  Measured:  `16689`
		//  Estimated: `206844`
		// Minimum execution time: 59_763_000 picoseconds.
		Weight::from_parts(60_188_000, 0)
			.saturating_add(Weight::from_parts(0, 206844))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_979, 0).saturating_mul(c.into()))
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
		//  Estimated: `3750`
		// Minimum execution time: 14_463_000 picoseconds.
		Weight::from_parts(14_800_000, 0)
			.saturating_add(Weight::from_parts(0, 3750))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(869, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	fn force_queue_action() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4251`
		//  Estimated: `13452`
		// Minimum execution time: 20_457_000 picoseconds.
		Weight::from_parts(21_033_000, 0)
			.saturating_add(Weight::from_parts(0, 13452))
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
		//  Estimated: `6986`
		// Minimum execution time: 8_834_000 picoseconds.
		Weight::from_parts(8_989_000, 0)
			.saturating_add(Weight::from_parts(0, 6986))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_961, 0).saturating_mul(c.into()))
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
		//  Estimated: `3521`
		// Minimum execution time: 6_731_000 picoseconds.
		Weight::from_parts(7_003_000, 0)
			.saturating_add(Weight::from_parts(0, 3521))
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
		//  Measured:  `26872`
		//  Estimated: `115408`
		// Minimum execution time: 91_664_000 picoseconds.
		Weight::from_parts(93_856_000, 0)
			.saturating_add(Weight::from_parts(0, 115408))
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
		//  Measured:  `27426`
		//  Estimated: `231783`
		// Minimum execution time: 778_654_000 picoseconds.
		Weight::from_parts(789_010_000, 0)
			.saturating_add(Weight::from_parts(0, 231783))
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
		//  Measured:  `27404`
		//  Estimated: `117536`
		// Minimum execution time: 91_553_000 picoseconds.
		Weight::from_parts(93_179_000, 0)
			.saturating_add(Weight::from_parts(0, 117536))
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
		//  Measured:  `26894`
		//  Estimated: `174234`
		// Minimum execution time: 617_068_000 picoseconds.
		Weight::from_parts(625_676_000, 0)
			.saturating_add(Weight::from_parts(0, 174234))
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
		//  Measured:  `26872`
		//  Estimated: `115408`
		// Minimum execution time: 90_504_000 picoseconds.
		Weight::from_parts(92_274_000, 0)
			.saturating_add(Weight::from_parts(0, 115408))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
