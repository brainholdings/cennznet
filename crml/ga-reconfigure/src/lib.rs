// Copyright 2018-2020 Parity Technologies (UK) Ltd. and Centrality Investments Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! # Generic Asset Reconfigure
//!
//! This module sets up the generic asset module according to a new configuration

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_event, decl_module, dispatch::Vec, weights::SimpleDispatchInfo, IterableStorageDoubleMap};
use frame_system::ensure_root;

use pallet_generic_asset::{FreeBalance, Module as GenericAsset};

decl_event! {
	pub enum Event<T> where <T as pallet_generic_asset::Trait>::AssetId {
		/// Burnt all tokens of an asset
		BurntOldTokens(AssetId),
		/// Minted new tokens
		MintedNewTokens,
	}
}

pub trait Trait: pallet_generic_asset::Trait + pallet_sudo::Trait {
	/// The event type of this module.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin, system = frame_system {

		fn deposit_event() = default;

		#[weight = SimpleDispatchInfo::FixedNormal(0)]
		pub fn exclusive_mint(origin, mint_list: Vec<(T::AssetId, T::AccountId, T::Balance)>) {
			ensure_root(origin.clone())?;

			let burn_tokens = |asset_id| {
				let balances_iter =
					<FreeBalance<T> as IterableStorageDoubleMap<T::AssetId, T::AccountId, T::Balance>>::iter(asset_id);
				balances_iter.for_each(|(who, balance)| {
					let _ = GenericAsset::<T>::burn_free(&asset_id, &pallet_sudo::Module::<T>::key(), &who, &balance);
				});
				Self::deposit_event(Event::<T>::BurntOldTokens(asset_id));
			};

			burn_tokens(GenericAsset::<T>::spending_asset_id());
			burn_tokens(GenericAsset::<T>::staking_asset_id());

			mint_list.iter().for_each(|(asset_id, who, balance)|{
				let _ = GenericAsset::<T>::mint_free(&asset_id, &pallet_sudo::Module::<T>::key(), &who, &balance);
			});

			Self::deposit_event(Event::<T>::MintedNewTokens);
		}
	}
}
