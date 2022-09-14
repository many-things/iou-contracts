use cosmwasm_std::{attr, Env, MessageInfo, Uint128};
use noi_alias::{DepsMut, Response};
use noi_interface::{
    core,
    helpers::{NoiCore, NoiOracle},
};

use crate::{
    state::{Config, Position, State},
    ContractError,
};

pub fn borrow(
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
    if amount > position.borrowable_debt(rate, config.borrow_ltv)? {
        return Err(ContractError::OverflowBorrowCapacity {});
    }

    position.debt = position.debt.checked_add(amount)?;
    position.save(deps.storage)?;

    state.total_debt = state.total_debt.checked_add(amount)?;
    state.save(deps.storage)?;

    let callback = NoiCore(config.core).call(core::CallbackMsg::Borrow {
        owner: position.owner.into_string(),
        position_id,
        amount,
    })?;

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "borrow"),
            attr("owner", info.sender.into_string()),
            attr("position_id", position_id.to_string()),
            attr("amount", amount.to_string()),
            attr("rate", rate.to_string()),
        ])
        .add_message(callback))
}

pub fn repay(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    position_id: u64,
) -> Result<Response, ContractError> {
    let config = Config::load(deps.storage)?;
    let received = cw_utils::must_pay(&info, &config.debt_asset)?;

    let mut state =
        State::load(deps.storage)?.update_fee(&env.block.time, config.fee_multiplier)?;

    let mut position =
        Position::load(deps.storage, position_id)?.apply_fee(state.global_fee_index)?;
    if position.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if position.debt < received {
        return Err(ContractError::Overpaid {});
    }

    position.debt = position.debt.checked_sub(received)?;
    position.save(deps.storage)?;

    state.total_debt = state.total_debt.checked_sub(received)?;
    state.save(deps.storage)?;

    let rate = NoiOracle(config.oracle).get_rate(&deps.querier)?.rate;
    let callback = NoiCore(config.core).call(core::CallbackMsg::Repay {
        owner: position.owner.into_string(),
        amount: received,
        position_id,
    })?;

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "repay"),
            attr("owner", info.sender.into_string()),
            attr("position_id", position_id.to_string()),
            attr("amount", received.to_string()),
            attr("rate", rate.to_string()),
        ])
        .add_message(callback))
}
