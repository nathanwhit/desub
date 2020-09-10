// Copyright 2019 Parity Technologies (UK) Ltd.
// substrate-desub is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// substrate-desub is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-desub.  If not, see <http://www.gnu.org/licenses/>.

use super::ModuleTypes;
use crate::{Result, TypeRange};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Clone)]
pub struct Extrinsics(HashMap<String, Vec<TypeRange>>);

impl Extrinsics {
    pub fn new(raw_json: &str) -> Result<Self> {
        serde_json::from_str(raw_json).map_err(Into::into)
    }

    pub fn get_chain_types(&self, chain: &str, spec: u32) -> Option<&ModuleTypes> {
        self.0
            .get(chain)?
            .iter()
            .find(|f| crate::is_in_range(spec, f))
            .map(|o| &o.types)
    }
}
