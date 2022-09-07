use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub gov: Addr,
    pub pool_id: u64,
    pub quote_asset: String,
    pub base_asset: String,
    pub time_range: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
