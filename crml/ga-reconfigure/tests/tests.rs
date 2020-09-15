/* Copyright 2019-2020 Centrality Investments Limited
*
* Licensed under the LGPL, Version 3.0 (the "License");
* you may not use this file except in compliance with the License.
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
* You may obtain a copy of the License at the root of this project source code,
* or at:
*     https://centrality.ai/licenses/gplv3.txt
*     https://centrality.ai/licenses/lgplv3.txt
*/

#![cfg(test)]

use cennznet_primitives::types::{AccountId, AssetId, Balance};
use cennznet_testing::keyring::{alice, bob};
use frame_support::{StorageDoubleMap, StorageMap};
use mock::{ExtBuilder, Reconfigure, Test, PLUG_ASSET_ID, SPENDING_ASSET_ID, STAKING_ASSET_ID};

mod mock;

type Origin = <Test as frame_system::Trait>::Origin;

#[test]
fn reconfigure_ga_balances() {
	const INITIAL_BALANCE: Balance = 1000_000_000_000;
	const INITIAL_ISSUANCE: Balance = INITIAL_BALANCE * 1000;
	const ALICE_BALANCE: Balance = INITIAL_BALANCE * 11;
	const BOB_BALANCE: Balance = INITIAL_BALANCE * 23;

	ExtBuilder::default().sudoer(alice()).build().execute_with(|| {
		type TotalIssuance = pallet_generic_asset::TotalIssuance<Test>;
		TotalIssuance::insert(&STAKING_ASSET_ID, INITIAL_ISSUANCE);
		TotalIssuance::insert(&SPENDING_ASSET_ID, INITIAL_ISSUANCE);
		TotalIssuance::insert(&PLUG_ASSET_ID, INITIAL_ISSUANCE);

		type FreeBalance = pallet_generic_asset::FreeBalance<Test>;
		FreeBalance::insert::<AssetId, AccountId, Balance>(STAKING_ASSET_ID, alice(), ALICE_BALANCE);
		FreeBalance::insert::<AssetId, AccountId, Balance>(SPENDING_ASSET_ID, alice(), ALICE_BALANCE);
		FreeBalance::insert::<AssetId, AccountId, Balance>(PLUG_ASSET_ID, alice(), ALICE_BALANCE);
		FreeBalance::insert::<AssetId, AccountId, Balance>(STAKING_ASSET_ID, bob(), BOB_BALANCE);
		FreeBalance::insert::<AssetId, AccountId, Balance>(SPENDING_ASSET_ID, bob(), BOB_BALANCE);
		FreeBalance::insert::<AssetId, AccountId, Balance>(PLUG_ASSET_ID, bob(), BOB_BALANCE);

		let _ = Reconfigure::exclusive_mint(
			Origin::ROOT,
			vec![
				(STAKING_ASSET_ID, alice(), INITIAL_BALANCE * 2),
				(SPENDING_ASSET_ID, alice(), INITIAL_BALANCE * 3),
				(SPENDING_ASSET_ID, bob(), INITIAL_BALANCE),
			],
		);

		type GenericAsset = pallet_generic_asset::Module<Test>;
		assert_eq!(
			GenericAsset::free_balance(&STAKING_ASSET_ID, &alice()),
			INITIAL_BALANCE * 2
		);
		assert_eq!(
			GenericAsset::free_balance(&SPENDING_ASSET_ID, &alice()),
			INITIAL_BALANCE * 3
		);
		assert_eq!(GenericAsset::free_balance(&PLUG_ASSET_ID, &alice()), ALICE_BALANCE);

		assert_eq!(GenericAsset::free_balance(&STAKING_ASSET_ID, &bob()), 0);
		assert_eq!(GenericAsset::free_balance(&SPENDING_ASSET_ID, &bob()), INITIAL_BALANCE);
		assert_eq!(GenericAsset::free_balance(&PLUG_ASSET_ID, &bob()), BOB_BALANCE);
	});
}
