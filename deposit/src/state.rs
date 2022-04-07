use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};



// This struct counts deposits, so we can use id as key for our Map
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub deposit_count: i32,
}
// This struct store information for each deposits
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub id: String,
    pub owner: Addr,
    pub amount: Uint128,
}


pub const DEPOSITS: Map<(&Addr, &str), Deposit> = Map::new("deposit");
pub const CONFIG: Item<Config> = Item::new("config");
