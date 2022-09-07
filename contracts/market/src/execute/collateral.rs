use cosmwasm_std::{Env, MessageInfo, Uint128};
use noi_alias::{DepsMut, Response};
use noi_interface::helpers::NoiOracle;

use crate::{
    state::{Config, Position, State},
    ContractError,
};

pub fn lock(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    position_id: u64,
) -> Result<Response, ContractError> {
    let config = Config::load(deps.storage)?;
    let received = cw_utils::must_pay(&info, &config.collateral_asset)?;

    let mut state =
        State::load(deps.storage)?.update_fee(&env.block.time, config.fee_multiplier)?;

    let mut position =
        Position::load(deps.storage, position_id)?.apply_fee(state.global_fee_index)?;
    if position.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    position.collateral = position.collateral.checked_add(received)?;
    position.stored_fee_index = state.global_fee_index;
    position.save_with_id(deps.storage, position_id)?;

    state.total_collateral = state.total_collateral.checked_add(received)?;
    state.save(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "lock")
        .add_attribute("owner", info.sender.into_string())
        .add_attribute("position_id", position_id.to_string())
        .add_attribute("amount", received.to_string()))
}

pub fn unlock(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    position_id: u64,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = Config::load(deps.storage)?;

    let mut state =
        State::load(deps.storage)?.update_fee(&env.block.time, config.fee_multiplier)?;

    let mut position =
        Position::load(deps.storage, position_id)?.apply_fee(state.global_fee_index)?;
    if position.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let rate = NoiOracle(config.oracle).get_rate(&deps.querier)?.rate;
    if amount > position.unlockable_collateral(rate, config.borrow_ltv)? {
        return Err(ContractError::OverflowUnlockCapacity {});
    }

    position.collateral = position.collateral.checked_sub(amount)?;
    position.save(deps.storage)?;

    state.total_collateral = state.total_collateral.checked_sub(amount)?;
    state.save(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "unlock")
        .add_attribute("owner", info.sender.into_string())
        .add_attribute("position_id", position_id.to_string())
        .add_attribute("amount", amount.to_string())
        .add_attribute("rate", rate.to_string()))
}
