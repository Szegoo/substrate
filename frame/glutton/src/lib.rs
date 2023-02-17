// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! # Glutton Pallet
//!
//! Pallet that consumes `ref_time` and `proof_size` of a block. Based on the
//! `Compute` and `Storage` parameters the pallet consumes the adequate amount
//! of weight.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;

use blake2::{Blake2b512, Digest};
use frame_support::{pallet_prelude::*, weights::WeightMeter};
use frame_system::pallet_prelude::*;
use sp_runtime::{traits::Zero, Perbill};
use sp_std::{vec, vec::Vec};

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event {
		/// The pallet has been initialized by root.
		PalletInitialized,
		/// The computation limit has been updated by root.
		ComputationLimitSet { compute: Perbill },
		/// The storage limit has been updated by root.
		StorageLimitSet { storage: Perbill },
	}

	#[pallet::storage]
	pub(crate) type Compute<T: Config> = StorageValue<_, Perbill, ValueQuery>;

	#[pallet::storage]
	pub(crate) type Storage<T: Config> = StorageValue<_, Perbill, ValueQuery>;

	/// Storage map used for wasting proof size.
	///
	/// It contains no meaningful data - hence the name "Trash". The maximal number of entries is
	/// set to 65k, which is just below the next jump at 16^4. This is important to reduce the proof
	/// size benchmarking overestimate. The assumption here is that we won't have more than 65k *
	/// 1KiB = 65MiB of proof size wasting in practice. However, this limit is not enforces, so the
	/// pallet would also work out of the box with more entries, but its benchmarked proof weight
	/// would possibly be underestimated in that case.
	#[pallet::storage]
	pub(super) type TrashData<T: Config> = StorageMap<
		Hasher = Twox64Concat,
		Key = u32,
		Value = [u8; 1024],
		QueryKind = OptionQuery,
		MaxValues = ConstU32<65_000>,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn integrity_test() {
			assert!(
				!T::WeightInfo::waste_ref_time_iter(1).ref_time().is_zero(),
				"Weight zero; would get stuck in an infinite loop"
			);
			assert!(
				!T::WeightInfo::waste_proof_size_some(1).proof_size().is_zero(),
				"Weight zero; would get stuck in an infinite loop"
			);
		}

		fn on_idle(_: BlockNumberFor<T>, remaining_weight: Weight) -> Weight {
			let mut meter = WeightMeter::from_limit(remaining_weight);
			if !meter.check_accrue(T::WeightInfo::empty_on_idle()) {
				return T::WeightInfo::empty_on_idle()
			}

			let proof_size_limit = Storage::<T>::get().mul_floor(meter.remaining().proof_size());
			let computation_weight_limit =
				Compute::<T>::get().mul_floor(meter.remaining().ref_time());
			meter.check_accrue(Weight::from_parts(
				computation_weight_limit,
				proof_size_limit,
			));

			Self::waste_at_most_proof_size(&mut meter);
			Self::waste_at_most_ref_time(&mut meter);

			meter.consumed
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Initializes the pallet by writing into `TrashData`.
		///
		/// Only callable by Root. A good default for `trash_count` is `5_000`.
		#[pallet::call_index(0)]
		#[pallet::weight(T::DbWeight::get().writes((*trash_count).into()))]
		pub fn initialize_pallet(origin: OriginFor<T>, trash_count: u32) -> DispatchResult {
			ensure_root(origin)?;

			// Fill up the `TrashData` storage item.
			(0..trash_count).for_each(|i| TrashData::<T>::insert(i, &[i as u8; 1024]));

			Self::deposit_event(Event::PalletInitialized);
			Ok(())
		}

		/// Set the `Compute` storage value that determines how much of the
		/// block's weight `ref_time` to use during `on_idle`.
		///
		/// Only callable by Root.
		#[pallet::call_index(1)]
		#[pallet::weight(T::DbWeight::get().writes(1))]
		pub fn set_compute(origin: OriginFor<T>, compute: Perbill) -> DispatchResult {
			ensure_root(origin)?;
			Compute::<T>::set(compute);

			Self::deposit_event(Event::ComputationLimitSet { compute });
			Ok(())
		}

		/// Set the `Storage` storage value that determines the PoV size usage
		/// for each block.
		///
		/// Only callable by Root.
		#[pallet::call_index(2)]
		#[pallet::weight(T::DbWeight::get().writes(1))]
		pub fn set_storage(origin: OriginFor<T>, storage: Perbill) -> DispatchResult {
			ensure_root(origin)?;
			Storage::<T>::set(storage);

			Self::deposit_event(Event::StorageLimitSet { storage });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Wastes some `proof_size`. Receives a counter as an argument.
		pub(crate) fn waste_at_most_proof_size(meter: &mut WeightMeter) {
			let Ok(n) = Self::calculate_proof_size_iters(&meter) else {
				return;
			};

			meter.defensive_saturating_accrue(T::WeightInfo::waste_proof_size_some(n));

			(0..n).for_each(|i| {
				TrashData::<T>::get(i);
			});
		}

		/// Calculate how many times `waste_proof_size_some` should be called to full up `meter`.
		fn calculate_proof_size_iters(meter: &WeightMeter) -> Result<u32, ()> {
			let base = T::WeightInfo::waste_proof_size_some(0);
			let slope = T::WeightInfo::waste_proof_size_some(1).saturating_sub(base);

			let remaining = meter.remaining().saturating_sub(base);
			let iter_by_proof_size =
				remaining.proof_size().checked_div(slope.proof_size()).ok_or(())?;
			let iter_by_ref_time = remaining.ref_time().checked_div(slope.ref_time()).ok_or(())?;

			match iter_by_proof_size {
				0 => Err(()),
				i if i <= iter_by_ref_time => Ok(i as u32),
				_ => Err(()),
			}
		}

		/// Waste at most the remaining ref time weight of `meter`.
		///
		/// Tries to come as close to the limit as possible.
		pub(crate) fn waste_at_most_ref_time(meter: &mut WeightMeter) {
			let Ok(n) = Self::calculate_ref_time_iters(&meter) else {
				return;
			};
			meter.defensive_saturating_accrue(T::WeightInfo::waste_ref_time_iter(n));

			let clobber = Self::waste_ref_time_iter(vec![0u8; 64], n);

			// By casting it into a vec we can hopefully prevent the compiler from optimizing it
			// out. Note that `Blake2b512` produces 64 bytes, this is therefore impossible - but the
			// compiler does not know that (hopefully).
			debug_assert!(clobber.len() == 64);
			if clobber.len() == 65 {
				TrashData::<T>::insert(0, [clobber[0] as u8; 1024]);
			}
		}

		/// Wastes some `ref_time`. Receives the previous result as an argument.
		///
		/// The ref_time of one iteration should be in the order of 1-10 ms.
		pub(crate) fn waste_ref_time_iter(clobber: Vec<u8>, i: u32) -> Vec<u8> {
			let mut hasher = Blake2b512::new();

			// Blake2 has a very high speed of hashing so we make multiple hashes with it to
			// waste more `ref_time` at once.
			(0..i).for_each(|_| {
				hasher.update(clobber.as_slice());
			});

			hasher.finalize().to_vec()
		}

		/// Calculate how many times `waste_ref_time_iter` should be called to full up `meter`.
		fn calculate_ref_time_iters(meter: &WeightMeter) -> Result<u32, ()> {
			let base = T::WeightInfo::waste_ref_time_iter(0);
			let slope = T::WeightInfo::waste_ref_time_iter(1).saturating_sub(base);
			if !slope.proof_size().is_zero() || !base.proof_size().is_zero() {
				return Err(())
			}

			match meter
				.remaining()
				.ref_time()
				.saturating_sub(base.ref_time())
				.checked_div(slope.ref_time())
			{
				Some(0) | None => Err(()),
				Some(i) => Ok(i as u32),
			}
		}
	}
}
