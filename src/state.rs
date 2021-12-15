use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Primitive {
    Uint128(Uint128),
    String(String),
    Bool(bool),
}

pub const DATA: Map<(&Addr, &str), Primitive> = Map::new("data");
pub const CONFIG: Item<Config> = Item::new("config");
