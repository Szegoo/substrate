// This file is part of Substrate.

// Copyright (C) 2023 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_glutton
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-27, STEPS: `20`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Sergejs-MacBook-Air.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/substrate
// benchmark
// pallet
// --chain=dev
// --steps=20
// --pallet=pallet_glutton
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/glutton/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs
// --extrinsic=*

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_glutton.
pub trait WeightInfo {
	fn waste_ref_time(n: u32, ) -> Weight;
	fn waste_proof_size_some(n: u32, ) -> Weight;
	fn waste_proof_size_none(n: u32, ) -> Weight;
	fn on_idle() -> Weight;
	fn empty_on_idle() -> Weight;
}

/// Weights for pallet_glutton using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `n` is `[0, 1024]`.
	fn waste_ref_time(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(7_942_950)
			// Standard Error: 157
			.saturating_add(Weight::from_ref_time(209).saturating_mul(n.into()))
	}
	/// Storage: Glutton TrashData (r:1 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1024]`.
	fn waste_proof_size_some(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `762`
		//  Estimated: `2499`
		// Minimum execution time: 3_000 nanoseconds.
		Weight::from_parts(6_773_699, 2499)
			// Standard Error: 1_126
			.saturating_add(Weight::from_ref_time(2_788).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Glutton TrashData (r:1 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1024]`.
	fn waste_proof_size_none(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `55`
		//  Estimated: `2499`
		// Minimum execution time: 3_000 nanoseconds.
		Weight::from_parts(3_100_346, 2499)
			// Standard Error: 341
			.saturating_add(Weight::from_ref_time(683).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:20 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12180`
		//  Estimated: `50978`
		// Minimum execution time: 78_503_000 nanoseconds.
		Weight::from_parts(78_503_000_000, 50978)
			.saturating_add(T::DbWeight::get().reads(22_u64))
	}
	fn empty_on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_000 nanoseconds.
		Weight::from_ref_time(1_000_000)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `n` is `[0, 1024]`.
	fn waste_ref_time(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(7_942_950)
			// Standard Error: 157
			.saturating_add(Weight::from_ref_time(209).saturating_mul(n.into()))
	}
	/// Storage: Glutton TrashData (r:1 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1024]`.
	fn waste_proof_size_some(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `762`
		//  Estimated: `2499`
		// Minimum execution time: 3_000 nanoseconds.
		Weight::from_parts(6_773_699, 2499)
			// Standard Error: 1_126
			.saturating_add(Weight::from_ref_time(2_788).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Glutton TrashData (r:1 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1024]`.
	fn waste_proof_size_none(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `55`
		//  Estimated: `2499`
		// Minimum execution time: 3_000 nanoseconds.
		Weight::from_parts(3_100_346, 2499)
			// Standard Error: 341
			.saturating_add(Weight::from_ref_time(683).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:20 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12180`
		//  Estimated: `50978`
		// Minimum execution time: 78_503_000 nanoseconds.
		Weight::from_parts(78_503_000_000, 50978)
			.saturating_add(RocksDbWeight::get().reads(22_u64))
	}
	fn empty_on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_000 nanoseconds.
		Weight::from_ref_time(1_000_000)
	}
}
