
//! Autogenerated weights for pallet_glutton
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `oty-parity`, CPU: `11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-glutton
// --extrinsic=*
// --execution=Native
// --wasm-execution=compiled
// --heap-pages=4096
// --template
// .maintain/frame-weight-template.hbs
// --output
// frame/glutton/src/weights.rs
// --extra

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_glutton.
pub trait WeightInfo {
	fn waste_ref_time_iter() -> Weight;
	fn waste_proof_size_some() -> Weight;
	fn waste_proof_size_none() -> Weight;
	fn read_limits() -> Weight;
	fn on_idle() -> Weight;
	fn empty_on_idle() -> Weight;
}

/// Weights for pallet_glutton using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn waste_ref_time_iter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_548_420 nanoseconds.
		Weight::from_ref_time(7_696_066_000)
	}
	/// Storage: Glutton TrashData (r:1 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn waste_proof_size_some() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1521`
		//  Estimated: `2499`
		// Minimum execution time: 13_469 nanoseconds.
		Weight::from_parts(14_549_000, 2499)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Glutton TrashData (r:1 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn waste_proof_size_none() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `55`
		//  Estimated: `2499`
		// Minimum execution time: 3_061 nanoseconds.
		Weight::from_parts(3_157_000, 2499)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn read_limits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `55`
		//  Estimated: `998`
		// Minimum execution time: 3_316 nanoseconds.
		Weight::from_parts(3_434_000, 998)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:2097 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `124221`
		//  Estimated: `5241401`
		// Minimum execution time: 24_287_754 nanoseconds.
		Weight::from_parts(24_680_302_000, 5241401)
			.saturating_add(T::DbWeight::get().reads(2099_u64))
	}
	fn empty_on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 417 nanoseconds.
		Weight::from_ref_time(457_000)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn waste_ref_time_iter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_548_420 nanoseconds.
		Weight::from_ref_time(7_696_066_000)
	}
	/// Storage: Glutton TrashData (r:1 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn waste_proof_size_some() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1521`
		//  Estimated: `2499`
		// Minimum execution time: 13_469 nanoseconds.
		Weight::from_parts(14_549_000, 2499)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Glutton TrashData (r:1 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn waste_proof_size_none() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `55`
		//  Estimated: `2499`
		// Minimum execution time: 3_061 nanoseconds.
		Weight::from_parts(3_157_000, 2499)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn read_limits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `55`
		//  Estimated: `998`
		// Minimum execution time: 3_316 nanoseconds.
		Weight::from_parts(3_434_000, 998)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:2097 w:0)
	/// Proof: Glutton TrashData (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `124221`
		//  Estimated: `5241401`
		// Minimum execution time: 24_287_754 nanoseconds.
		Weight::from_parts(24_680_302_000, 5241401)
			.saturating_add(RocksDbWeight::get().reads(2099_u64))
	}
	fn empty_on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 417 nanoseconds.
		Weight::from_ref_time(457_000)
	}
}
