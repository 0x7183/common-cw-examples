use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

<<<<<<< HEAD
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
=======
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
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
