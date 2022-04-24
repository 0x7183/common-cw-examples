use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr};
use cosmwasm_bignumber::Uint256;
use cw_storage_plus::{Item, Map};

// Struct for deposit
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PoolInfo {
    pub id: String,
    pub amount: Uint256,
    pub denom: String,
    pub aust_amount: Uint256,
    pub exchange_rate_prev: String,
    pub depositor_addr: String,
}

impl PoolInfo {
    pub fn get_amount(&self) {
        self.amount;
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Config {
    pub deposit_count: i32,
    pub stable_denom: String,
    pub aterra_address: CanonicalAddr,
    pub moneymarket: CanonicalAddr
}


pub const DEPOSITORS: Map<(&str, &str), PoolInfo> = Map::new("depositors");
pub const CONFIG: Item<Config> = Item::new("config");
