// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `runtime_common::coretime`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-bn-ce5rx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("rococo-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=runtime_common::coretime
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./polkadot/file_header.txt
// --output=./polkadot/runtime/rococo/src/weights/runtime_common_coretime.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_common::coretime`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::coretime::WeightInfo for WeightInfo<T> {
	/// Storage: `Configuration::PendingConfigs` (r:1 w:1)
	/// Proof: `Configuration::PendingConfigs` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Configuration::BypassConsistencyCheck` (r:1 w:0)
	/// Proof: `Configuration::BypassConsistencyCheck` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn request_core_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `151`
		//  Estimated: `1636`
		// Minimum execution time: 7_543_000 picoseconds.
		Weight::from_parts(7_745_000, 0)
			.saturating_add(Weight::from_parts(0, 1636))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CoretimeAssignmentProvider::CoreDescriptors` (r:1 w:1)
	/// Proof: `CoretimeAssignmentProvider::CoreDescriptors` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CoretimeAssignmentProvider::CoreSchedules` (r:0 w:1)
	/// Proof: `CoretimeAssignmentProvider::CoreSchedules` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[1, 100]`.
	fn assign_core(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `180`
		//  Estimated: `3645`
		// Minimum execution time: 9_367_000 picoseconds.
		Weight::from_parts(9_932_305, 0)
			.saturating_add(Weight::from_parts(0, 3645))
			// Standard Error: 231
			.saturating_add(Weight::from_parts(12_947, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}