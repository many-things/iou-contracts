use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Empty, StdResult, Storage};
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("item::config");

pub const MARKET: Map<&Addr, Market> = Map::new("map::market");
pub const IDX_MARKET_BY_NAME: Map<&str, Addr> = Map::new("map::idx_market_by_category");
pub const IDX_MARKET_BY_COLLATERAL: Map<(&str, &Addr), Empty> =
    Map::new("map::idx_market_by_collateral");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub gov: Addr,
    pub denom: String,
    pub treasury: Addr,
    pub liquidator: Addr,
}

impl Config {
    pub fn save(self, storage: &mut dyn Storage) -> StdResult<Self> {
        CONFIG.save(storage, &self)?;
        Ok(self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Config> {
        CONFIG.load(storage)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Market {
    pub name: String,
    pub collateral: String,
}

impl Market {
    pub fn raw_map() -> Map<'static, &'static Addr, Self> {
        MARKET
    }

    pub fn raw_idx_name() -> Map<'static, &'static str, Addr> {
        IDX_MARKET_BY_NAME
    }

    pub fn raw_idx_collateral() -> Map<'static, (&'static str, &'static Addr), Empty> {
        IDX_MARKET_BY_COLLATERAL
    }
}

impl Market {
    pub fn save(self, storage: &mut dyn Storage, address: &Addr) -> StdResult<Self> {
        MARKET.save(storage, address, &self)?;
        IDX_MARKET_BY_NAME.save(storage, &self.name, address)?;
        IDX_MARKET_BY_COLLATERAL.save(storage, (&self.collateral, address), &Empty {})?;

        Ok(self)
    }

    pub fn load(storage: &dyn Storage, address: &Addr) -> StdResult<Self> {
        MARKET.load(storage, address)
    }

    pub fn remove(self, storage: &mut dyn Storage, address: &Addr) -> StdResult<Self> {
        MARKET.remove(storage, address);
        IDX_MARKET_BY_NAME.remove(storage, &self.name);
        IDX_MARKET_BY_COLLATERAL.remove(storage, (&self.collateral, address));

        Ok(self)
    }
}
