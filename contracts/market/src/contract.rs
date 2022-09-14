#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Env, MessageInfo, StdResult, Uint128};
use noi_alias::{Deps, DepsMut, Response};
use noi_interface::market::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::error::ContractError;
use crate::state::{Config, Position, State};

const CONTRACT_NAME: &str = "crates.io:noi-market";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Config {
        name: msg.name,

        core: deps.api.addr_validate(&msg.core)?,
        oracle: deps.api.addr_validate(&msg.oracle)?,
        liquidator: deps.api.addr_validate(&msg.liquidator)?,

        debt_asset: msg.debt_asset,
        collateral_asset: msg.collateral_asset,

        borrow_ltv: msg.borrow_ltv,
        fee_multiplier: msg.fee_multiplier,
    }
    .save(deps.storage)?;

    State {
        total_debt: Uint128::zero(),
        total_collateral: Uint128::zero(),
        fee_updated_at: env.block.time.seconds(),
        global_fee_index: msg.fee_base,
    }
    .save(deps.storage)?;

    Position::raw_count().save(deps.storage, &u64::default())?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute;
    use ExecuteMsg::*;

    match msg {
        Config(config_msg) => execute::config(deps, env, info, config_msg),

        // position
        Open {} => execute::open(deps, env, info),
        Close { position_id } => execute::close(deps, env, info, position_id),
        Liquidate { position_id } => execute::liquidate(deps, env, info, position_id),

        // collateral
        Lock { position_id } => execute::lock(deps, env, info, position_id),
        Unlock {
            position_id,
            amount,
        } => execute::unlock(deps, env, info, position_id, amount),

        // debt
        Borrow {
            position_id,
            amount,
        } => execute::borrow(deps, env, info, position_id, amount),
        Repay { position_id } => execute::repay(deps, env, info, position_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use crate::query;
    use QueryMsg::*;

    match msg {
        GetConfig {} => query::get_config(deps),
        GetState {} => query::get_state(deps),
        GetPosition { position_id } => query::get_position(deps, position_id),
        ListPosition(opt) => query::list_position(deps, opt),
    }
}

#[cfg(test)]
mod tests {}
