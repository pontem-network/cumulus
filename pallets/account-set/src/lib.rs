// Copyright 2019-2020 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Small pallet responsible storing a set of accounts. In principal the uses are endless, but in
//! practice this is used as a minimal solution where staking would be used in practice.
//! The accounts are set and genesis and never change.
//!
//! The Substrate ecosystem has a wide variety of real-world solutions and examples of what this
//! pallet could be replaced with. (IOU Links)
//! Gautam's POA pallet
//! Parity's pallet staking
//! Moonbeam's Pallet Stake
//! Recipe for AccountSet, VecSet, and MapSet

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;

pub use pallet::*;

#[pallet]
pub mod pallet {

	use frame_support::debug::warn;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::Vec;
	use frame_system::pallet_prelude::*;

	/// The Account Set pallet
	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	/// Configuration trait of this pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config  {}

	// No hooks
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// No calls
	//TODO can I remove this entirely? I suspect not because of the hooks thing above.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	/// A vector of accounts. I hadn't origianlly intended for duplicates to exist, and now that
	/// I'm thinking about it, I can see some usecases for having dupes (higher probability of
	/// being selected in some filters), so I'm not going to enforce anything.
	#[pallet::storage]
	pub type StoredAccounts<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	impl<T: Config> Get<Vec<T::AccountId>> for Pallet<T> {
		fn get() -> Vec<T::AccountId> {
			StoredAccounts::<T>::get()
		}
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub stored_accounts: Vec<T::AccountId>,
	}

	//TODO can I derive default?
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				//TODO It would nice to somehow put alice and bob here, but that requires knowledge
				// of the concrete AccountId type. Maybe I could take it in the config trait and
				// let the runtime author supply it, but for now, I'm gonna do it in chain_spec.rs
				stored_accounts: Vec::new(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {

			if self.stored_accounts.is_empty() {
				warn!(target: "account-set", "No accounts stored at genesis. If this is used for authorship, your chain will have no valid authors.");
			}
			StoredAccounts::<T>::put(&self.stored_accounts);
		}
	}
}
