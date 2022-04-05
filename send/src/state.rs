use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Map, Item};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct Config{
    pub id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct History {
    pub id: String,
    pub sender: Addr,
    pub receiver: Addr,
    pub amount: Uint128,
}

pub const HISTORY: Map<(&Addr, &str),History> = Map::new("history");
pub const CONFIG: Item<Config> = Item::new("config");
