// This file is part of Substrate.

// Copyright (C) 2019-2020 Parity Technologies (UK) Ltd.
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

//! On-chain logic to store a validator-set for deferred validation using an off-chain worker.

use codec::Encode;
use sp_runtime::traits::Convert;

use super::super::Trait as SessionTrait;
use super::super::{Module as SessionModule, SessionIndex};
use super::Trait as HistoricalTrait;

use super::shared;

/// Store the validator-set associated to the `session_index` to the off-chain database.
///
/// Further processing is then done [`off-chain side`](super::offchain).
///
/// **Must** be called from on-chain, i.e. `on_initialize(..)` or `on_finalization(..)`.
pub fn store_session_validator_set_to_offchain<T: HistoricalTrait + SessionTrait>(
	session_index: SessionIndex,
) {
	//let value = SessionModule::historical_root(session_index);
	let encoded_validator_list = <SessionModule<T>>::validators()
		.into_iter()
		.filter_map(|validator_id: <T as SessionTrait>::ValidatorId| {
			let full_identification =
				<<T as HistoricalTrait>::FullIdentificationOf>::convert(validator_id.clone());
			full_identification.map(|full_identification| (validator_id, full_identification))
		})
		.collect::<Vec<_>>();

	encoded_validator_list.using_encoded(|encoded_validator_list| {
		let derived_key = shared::derive_key(shared::PREFIX, session_index);
		sp_io::offchain_index::set(derived_key.as_slice(), encoded_validator_list);
	});
}

/// Store the validator set associated to the _current_ session index to the off-chain database.
///
/// See [`fn store_session_validator_set_to_offchain(..)`](Self::store_session_validator_set_to_offchain)
/// for further information and restrictions.
pub fn store_current_session_validator_set_to_offchain<T: HistoricalTrait + SessionTrait>() {
	store_session_validator_set_to_offchain::<T>(<SessionModule<T>>::current_index());
}