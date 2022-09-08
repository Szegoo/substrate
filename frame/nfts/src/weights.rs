// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_nfts
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// /home/benchbot/cargo_target_dir/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_nfts
// --chain=dev
// --output=./frame/nfts/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nfts.
pub trait WeightInfo {
	fn create() -> Weight;
	fn force_create() -> Weight;
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight;
	fn mint() -> Weight;
	fn burn() -> Weight;
	fn transfer() -> Weight;
	fn redeposit(i: u32, ) -> Weight;
	fn freeze() -> Weight;
	fn thaw() -> Weight;
	fn freeze_collection() -> Weight;
	fn thaw_collection() -> Weight;
	fn transfer_ownership() -> Weight;
	fn set_team() -> Weight;
	fn force_item_status() -> Weight;
	fn set_attribute() -> Weight;
	fn clear_attribute() -> Weight;
	fn set_metadata() -> Weight;
	fn clear_metadata() -> Weight;
	fn set_collection_metadata() -> Weight;
	fn clear_collection_metadata() -> Weight;
	fn approve_transfer() -> Weight;
	fn cancel_approval() -> Weight;
	fn clear_all_transfer_approvals() -> Weight;
	fn set_accept_ownership() -> Weight;
	fn set_collection_max_supply() -> Weight;
	fn set_price() -> Weight;
	fn buy_item() -> Weight;
}

/// Weights for pallet_nfts using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassAccount (r:0 w:1)
	fn create() -> Weight {
		(32_564_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassAccount (r:0 w:1)
	fn force_create() -> Weight {
		(20_448_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts Asset (r:1 w:0)
	// Storage: Nfts ClassAccount (r:0 w:1)
	// Storage: Nfts Attribute (r:0 w:1000)
	// Storage: Nfts ClassMetadataOf (r:0 w:1)
	// Storage: Nfts InstanceMetadataOf (r:0 w:1000)
	// Storage: Nfts CollectionMaxSupply (r:0 w:1)
	// Storage: Nfts Account (r:0 w:20)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 13_000
			.saturating_add((9_842_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 13_000
			.saturating_add((1_695_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 13_000
			.saturating_add((1_617_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts CollectionMaxSupply (r:1 w:0)
	// Storage: Nfts Account (r:0 w:1)
	fn mint() -> Weight {
		(41_868_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Account (r:0 w:1)
	// Storage: Nfts ItemPriceOf (r:0 w:1)
	fn burn() -> Weight {
		(42_815_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Account (r:0 w:2)
	// Storage: Nfts ItemPriceOf (r:0 w:1)
	fn transfer() -> Weight {
		(33_593_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts Asset (r:102 w:102)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 12_000
			.saturating_add((11_005_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Class (r:1 w:0)
	fn freeze() -> Weight {
		(26_807_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Class (r:1 w:0)
	fn thaw() -> Weight {
		(26_654_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	fn freeze_collection() -> Weight {
		(22_118_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	fn thaw_collection() -> Weight {
		(21_747_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts OwnershipAcceptance (r:1 w:1)
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassAccount (r:0 w:2)
	fn transfer_ownership() -> Weight {
		(30_510_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	fn set_team() -> Weight {
		(23_375_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassAccount (r:0 w:1)
	fn force_item_status() -> Weight {
		(24_905_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts InstanceMetadataOf (r:1 w:0)
	// Storage: Nfts Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		(47_436_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts InstanceMetadataOf (r:1 w:0)
	// Storage: Nfts Attribute (r:1 w:1)
	fn clear_attribute() -> Weight {
		(46_314_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts InstanceMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		(39_100_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts InstanceMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		(40_551_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassMetadataOf (r:1 w:1)
	fn set_collection_metadata() -> Weight {
		(38_086_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts ClassMetadataOf (r:1 w:1)
	fn clear_collection_metadata() -> Weight {
		(37_407_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Asset (r:1 w:1)
	fn approve_transfer() -> Weight {
		(29_241_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Asset (r:1 w:1)
	fn cancel_approval() -> Weight {
		(28_857_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Asset (r:1 w:1)
	fn clear_all_transfer_approvals() -> Weight {
		(27_779_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts OwnershipAcceptance (r:1 w:1)
	fn set_accept_ownership() -> Weight {
		(25_718_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts CollectionMaxSupply (r:1 w:1)
	// Storage: Nfts Class (r:1 w:0)
	fn set_collection_max_supply() -> Weight {
		(23_601_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Asset (r:1 w:0)
	// Storage: Nfts ItemPriceOf (r:0 w:1)
	fn set_price() -> Weight {
		(25_306_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts ItemPriceOf (r:1 w:1)
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Account (r:0 w:2)
	fn buy_item() -> Weight {
		(45_058_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassAccount (r:0 w:1)
	fn create() -> Weight {
		(32_564_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassAccount (r:0 w:1)
	fn force_create() -> Weight {
		(20_448_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts Asset (r:1 w:0)
	// Storage: Nfts ClassAccount (r:0 w:1)
	// Storage: Nfts Attribute (r:0 w:1000)
	// Storage: Nfts ClassMetadataOf (r:0 w:1)
	// Storage: Nfts InstanceMetadataOf (r:0 w:1000)
	// Storage: Nfts CollectionMaxSupply (r:0 w:1)
	// Storage: Nfts Account (r:0 w:20)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 13_000
			.saturating_add((9_842_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 13_000
			.saturating_add((1_695_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 13_000
			.saturating_add((1_617_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts CollectionMaxSupply (r:1 w:0)
	// Storage: Nfts Account (r:0 w:1)
	fn mint() -> Weight {
		(41_868_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Account (r:0 w:1)
	// Storage: Nfts ItemPriceOf (r:0 w:1)
	fn burn() -> Weight {
		(42_815_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Account (r:0 w:2)
	// Storage: Nfts ItemPriceOf (r:0 w:1)
	fn transfer() -> Weight {
		(33_593_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts Asset (r:102 w:102)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 12_000
			.saturating_add((11_005_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Class (r:1 w:0)
	fn freeze() -> Weight {
		(26_807_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts Class (r:1 w:0)
	fn thaw() -> Weight {
		(26_654_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	fn freeze_collection() -> Weight {
		(22_118_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	fn thaw_collection() -> Weight {
		(21_747_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts OwnershipAcceptance (r:1 w:1)
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassAccount (r:0 w:2)
	fn transfer_ownership() -> Weight {
		(30_510_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	fn set_team() -> Weight {
		(23_375_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassAccount (r:0 w:1)
	fn force_item_status() -> Weight {
		(24_905_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts InstanceMetadataOf (r:1 w:0)
	// Storage: Nfts Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		(47_436_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts InstanceMetadataOf (r:1 w:0)
	// Storage: Nfts Attribute (r:1 w:1)
	fn clear_attribute() -> Weight {
		(46_314_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts InstanceMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		(39_100_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts InstanceMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		(40_551_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:1)
	// Storage: Nfts ClassMetadataOf (r:1 w:1)
	fn set_collection_metadata() -> Weight {
		(38_086_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts ClassMetadataOf (r:1 w:1)
	fn clear_collection_metadata() -> Weight {
		(37_407_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Asset (r:1 w:1)
	fn approve_transfer() -> Weight {
		(29_241_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Asset (r:1 w:1)
	fn cancel_approval() -> Weight {
		(28_857_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Asset (r:1 w:1)
	fn clear_all_transfer_approvals() -> Weight {
		(27_779_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts OwnershipAcceptance (r:1 w:1)
	fn set_accept_ownership() -> Weight {
		(25_718_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts CollectionMaxSupply (r:1 w:1)
	// Storage: Nfts Class (r:1 w:0)
	fn set_collection_max_supply() -> Weight {
		(23_601_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Asset (r:1 w:0)
	// Storage: Nfts ItemPriceOf (r:0 w:1)
	fn set_price() -> Weight {
		(25_306_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Asset (r:1 w:1)
	// Storage: Nfts ItemPriceOf (r:1 w:1)
	// Storage: Nfts Class (r:1 w:0)
	// Storage: Nfts Account (r:0 w:2)
	fn buy_item() -> Weight {
		(45_058_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
}
