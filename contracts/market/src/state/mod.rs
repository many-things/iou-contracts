mod position;

use cosmwasm_std::{Addr, Decimal, Empty, StdResult, Storage, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use crate::state::position::Position;

pub const CONFIG: Item<Config> = Item::new("item::config");
pub const STATE: Item<State> = Item::new("item::state");

pub const POSITION_COUNT: Item<u64> = Item::new("item::position_count");
pub const POSITIONS: Map<u64, Position> = Map::new("map::position");
pub const IDX_POSITION_BY_OWNER: Map<(&Addr, u64), Empty> = Map::new("map::idx_position_by_owner");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub name: String,

    pub core: Addr,
    pub oracle: Addr,
    pub liquidator: Addr,

    pub debt_asset: String,
    pub collateral_asset: String,

    pub borrow_ltv: Decimal,
    pub fee_multiplier: Decimal, // compound by second
}

impl Config {
    pub fn save(self, storage: &mut dyn Storage) -> StdResult<Self> {
        CONFIG.save(storage, &self)?;
        Ok(self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Self> {
        CONFIG.load(storage)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_debt: Uint128,
    pub total_collateral: Uint128,
    pub fee_updated_at: u64,
    pub global_fee_index: Decimal,
}

impl State {
    pub fn update_fee(mut self, now: &Timestamp, multiplier: Decimal) -> StdResult<Self> {
        let elapsed = (now.seconds() - self.fee_updated_at) as u32;
        if elapsed == 0 {
            return Ok(self);
        }

        self.global_fee_index += self.global_fee_index
            * ((Decimal::one() + multiplier).checked_pow(elapsed)? - Decimal::one());

        self.fee_updated_at = now.seconds();

        Ok(self)
    }
}

impl State {
    pub fn save(self, storage: &mut dyn Storage) -> StdResult<Self> {
        STATE.save(storage, &self)?;
        Ok(self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Self> {
        STATE.load(storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_fee() {
        let mut state = State {
            total_debt: Uint128::zero(),
            total_collateral: Uint128::zero(),
            fee_updated_at: 0,
            global_fee_index: Decimal::one(),
        };

        let now = 86400 * 365; // 1y
        let multiplier = Decimal::from_ratio(1029528u128, 1000000000000000u128); // daily multiplier of apy 3.3%

        state = state
            .update_fee(&Timestamp::from_seconds(now), multiplier)
            .unwrap();

        assert_eq!(state.fee_updated_at, now);
        assert_eq!(
            state.global_fee_index.atomics() * Uint128::from(1000u128) / Decimal::one().atomics(),
            Uint128::from(1033u128)
        );
    }
}
