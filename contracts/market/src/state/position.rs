use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Empty, StdResult, Storage, Uint128};

use crate::ContractError;

use super::{IDX_POSITION_BY_OWNER, POSITIONS, POSITION_COUNT};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Position {
    pub owner: Addr,
    pub collateral: Uint128,
    pub debt: Uint128,

    pub stored_fee_index: Decimal,
}

impl Position {
    pub fn raw_map() -> Map<'static, u64, Self> {
        POSITIONS
    }

    pub fn raw_count() -> Item<'static, u64> {
        POSITION_COUNT
    }

    pub fn raw_idx_owner() -> Map<'static, (&'static Addr, u64), Empty> {
        IDX_POSITION_BY_OWNER
    }
}

impl Position {
    fn collateral_value(&self, rate: Decimal) -> Uint128 {
        self.collateral * rate
    }

    fn red_line(&self, ltv: Decimal) -> Uint128 {
        self.debt * ltv
    }

    pub fn is_liquidatable(&self, rate: Decimal, ltv: Decimal) -> bool {
        self.collateral_value(rate) < self.red_line(ltv)
    }

    pub fn unlockable_collateral(
        &self,
        rate: Decimal,
        ltv: Decimal,
    ) -> Result<Uint128, ContractError> {
        Ok(self
            .collateral
            .checked_sub(self.red_line(ltv) * Decimal::one().atomics() / rate.atomics())?)
    }

    pub fn borrowable_debt(&self, rate: Decimal, ltv: Decimal) -> Result<Uint128, ContractError> {
        Ok(self
            .collateral_value(rate)
            .checked_sub(self.red_line(ltv))?)
    }
}

// state actions
impl Position {
    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<u64> {
        let position_count = POSITION_COUNT.load(storage)?;
        POSITIONS.save(storage, position_count, self)?;
        IDX_POSITION_BY_OWNER.save(storage, (&self.owner, position_count), &Empty {})?;
        POSITION_COUNT.save(storage, &(position_count + 1))?;
        Ok(position_count)
    }

    pub fn save_with_id(&self, storage: &mut dyn Storage, position_id: u64) -> StdResult<()> {
        POSITIONS.save(storage, position_id, self)
    }

    pub fn load(storage: &dyn Storage, position_id: u64) -> StdResult<Self> {
        POSITIONS.load(storage, position_id)
    }

    pub fn apply_fee(mut self, global_fee_index: Decimal) -> StdResult<Self> {
        self.debt = self
            .debt
            .checked_add(self.debt * (global_fee_index - self.stored_fee_index))?;
        self.stored_fee_index = global_fee_index;
        Ok(self)
    }

    pub fn close(storage: &mut dyn Storage, position_id: u64) -> StdResult<()> {
        let position = POSITIONS.load(storage, position_id)?;
        POSITIONS.remove(storage, position_id);
        IDX_POSITION_BY_OWNER.remove(storage, (&position.owner, position_id));
        Ok(())
    }

    pub fn transfer(storage: &mut dyn Storage, position_id: u64, new_owner: Addr) -> StdResult<()> {
        let mut position = POSITIONS.load(storage, position_id)?;

        IDX_POSITION_BY_OWNER.remove(storage, (&position.owner, position_id));
        IDX_POSITION_BY_OWNER.save(storage, (&new_owner, position_id), &Empty {})?;

        position.owner = new_owner;
        POSITIONS.save(storage, position_id, &position)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_position() -> Position {
        Position {
            owner: Addr::unchecked("owner"),
            collateral: Uint128::from(100u128),
            debt: Uint128::from(60u128),
            stored_fee_index: Decimal::one(),
        }
    }

    #[test]
    fn test_unlockable_collateral() {
        let position = base_position();
        let cases = [
            (300, 200, 60),
            (200, 200, 40),
            (300, 300, 40),
            (200, 300, 10),
        ];

        for (rate, ltv, expected) in cases {
            assert_eq!(
                position
                    .unlockable_collateral(Decimal::percent(rate), Decimal::percent(ltv))
                    .unwrap(),
                Uint128::from(expected as u128)
            );
        }
    }

    #[test]
    fn test_borrowable_debt() {
        let position = base_position();
        let cases = [
            (300, 200, 180),
            (200, 200, 80),
            (300, 300, 120),
            (200, 300, 20),
        ];

        for (rate, ltv, expected) in cases {
            assert_eq!(
                position
                    .borrowable_debt(Decimal::percent(rate), Decimal::percent(ltv))
                    .unwrap(),
                Uint128::from(expected as u128)
            );
        }
    }
}
