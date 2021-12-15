use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Primitive {
    Uint128(Uint128),
    String(String),
    Bool(bool),
}

pub const DATA: Map<Addr, HashMap<String, Primitive>> = Map::new("data");
pub const STATE: Item<State> = Item::new("state");
