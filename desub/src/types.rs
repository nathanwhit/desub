// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-desub.
//
// substrate-desub is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// substrate-desub is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-desub.  If not, see <http://www.gnu.org/licenses/>.

use desub_current::decoder::{Extrinsic, StorageEntry};
use desub_legacy::decoder::{GenericExtrinsic, GenericStorage};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum LegacyOrCurrent<L, C> {
	Legacy(L),
	Current(C),
}

pub type LegacyOrCurrentExtrinsic = LegacyOrCurrent<GenericExtrinsic, Extrinsic<'static>>;

pub type LegacyOrCurrentStorage = LegacyOrCurrent<GenericStorage, StorageEntry<'static, 'static>>;

impl LegacyOrCurrentStorage {
	pub fn module(&self) -> String {
		match self {
			LegacyOrCurrent::Current(entry) => entry.prefix.clone().into(),
			LegacyOrCurrent::Legacy(legacy) => legacy.key().module.clone(),
		}
	}

	pub fn name(&self) -> String {
		match self {
			LegacyOrCurrent::Current(entry) => entry.name.clone().into(),
			LegacyOrCurrent::Legacy(legacy) => legacy.key().prefix.clone(),
		}
	}
}
