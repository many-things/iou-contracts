use cosmwasm_std::{coins, BankMsg, Env, MessageInfo, Uint128};
use noi_alias::{DepsMut, Response};
use noi_interface::{
    core,
    helpers::{NoiCore, NoiOracle},
};

use crate::{
    state::{Config, Position, State},
    ContractError,
};

pub fn open(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = Config::load(deps.storage)?;
    let state = State::load(deps.storage)?
        .update_fee(&env.block.time, config.fee_multiplier)?
        .save(deps.storage)?;

    let position_id = Position {
        owner: info.sender.clone(),
        collateral: Uint128::zero(),
        debt: Uint128::zero(),

        stored_fee_index: state.global_fee_index,
    }
    .save(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "open")
        .add_attribute("owner", info.sender)
        .add_attribute("position_id", position_id.to_string()))
}

pub fn close(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    position_id: u64,
) -> Result<Response, ContractError> {
    let config = Config::load(deps.storage)?;
    let mut state =
        State::load(deps.storage)?.update_fee(&env.block.time, config.fee_multiplier)?;

    let position = Position::load(deps.storage, position_id)?.apply_fee(state.global_fee_index)?;
    if position.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if !position.debt.is_zero() {
        return Err(ContractError::NotLiquidated {});
    }

    Position::close(deps.storage, position_id)?;

    state.total_collateral = state.total_collateral.checked_sub(position.collateral)?;
    state.save(deps.storage)?;

    let return_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(position.collateral.u128(), config.collateral_asset),
    };

    Ok(Response::new()
        .add_attribute("action", "close")
        .add_attribute("owner", info.sender.into_string())
        .add_attribute("position_id", position_id.to_string())
        .add_message(return_msg))
}

pub fn liquidate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    position_id: u64,
) -> Result<Response, ContractError> {
    let config = Config::load(deps.storage)?;
    let mut state =
        State::load(deps.storage)?.update_fee(&env.block.time, config.fee_multiplier)?;

    let rate = NoiOracle(config.oracle).get_rate(&deps.querier)?.rate;
    let position = Position::load(deps.storage, position_id)?.apply_fee(state.global_fee_index)?;
    if !position.is_liquidatable(rate, config.borrow_ltv) {
        return Err(ContractError::NotLiquidatable {});
    }

    Position::close(deps.storage, position_id)?;

    state.total_collateral = state.total_collateral.checked_sub(position.collateral)?;
    state.total_debt = state.total_debt.checked_sub(position.debt)?;
    state.save(deps.storage)?;

    let after_liquidation = NoiCore(config.core).call(core::InternalMsg::AfterLiquidation {
        owner: position.owner.to_string(),
        asset: config.collateral_asset,
        collateral: position.collateral,
        debt: position.debt,
    })?;

    Ok(Response::new()
        .add_attribute("action", "liquidate")
        .add_attribute("owner", position.owner.into_string())
        .add_attribute("position_id", position_id.to_string())
        .add_attribute("collateral", position.collateral.to_string())
        .add_attribute("debt", position.debt.to_string())
        .add_attribute("rate", rate.to_string())
        .add_message(after_liquidation))
}
