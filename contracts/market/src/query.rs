use cosmwasm_std::{to_binary, Binary, Order, StdError, StdResult};
use cw_storage_plus::Bound;
use noi_alias::Deps;
use noi_interface::{
    market::{
        GetConfigResponse, GetPositionResponse, GetStateResponse, ListPositionMsg,
        ListPositionResponse,
    },
    RangeOrder, DEFAULT_LIMIT, MAX_LIMIT,
};

use crate::{
    state::{Config, Position, State},
    ContractError,
};

pub fn get_config(deps: Deps) -> StdResult<Binary> {
    let config = Config::load(deps.storage)?;

    to_binary(&GetConfigResponse {
        name: config.name,

        core: config.core.into_string(),
        oracle: config.oracle.into_string(),

        debt_asset: config.debt_asset,
        collateral_asset: config.collateral_asset,

        borrow_ltv: config.borrow_ltv,
        fee_multiplier: config.fee_multiplier.clone(),
        fee_apy: config.fee_multiplier.checked_pow(86400 * 365)?, // 1y,
    })
}

pub fn get_state(deps: Deps) -> StdResult<Binary> {
    let state = State::load(deps.storage)?;

    to_binary(&GetStateResponse {
        total_debt: state.total_debt,
        total_collateral: state.total_collateral,
        fee_updated_at: state.fee_updated_at,
        global_fee_index: state.global_fee_index,
    })
}

pub fn get_position(deps: Deps, position_id: u64) -> StdResult<Binary> {
    let position = Position::load(deps.storage, position_id)?;
    let state = State::load(deps.storage)?;

    let position_with_fee = position.clone().apply_fee(state.global_fee_index)?;

    to_binary(&GetPositionResponse {
        id: position_id,
        owner: position.owner.into_string(),
        collateral: position.collateral,
        debt: position.debt,
        pending_fee: position_with_fee.debt.checked_sub(position.debt)?,
    })
}

pub fn list_position(deps: Deps, opt: ListPositionMsg) -> StdResult<Binary> {
    let state = State::load(deps.storage)?;

    match opt {
        ListPositionMsg::Default {
            start_after,
            limit,
            order,
        } => {
            let responses: StdResult<Vec<GetPositionResponse>> =
                Position::list(deps.storage, start_after, limit, order)?
                    .iter()
                    .map(|(id, position)| {
                        let position_with_fee =
                            position.clone().apply_fee(state.global_fee_index)?;

                        Ok(GetPositionResponse {
                            id: *id,
                            owner: position.owner.to_string(),
                            collateral: position.collateral,
                            debt: position.debt,
                            pending_fee: position_with_fee.debt.checked_sub(position.debt)?,
                        })
                    })
                    .collect();

            to_binary(&ListPositionResponse(responses?))
        }
        ListPositionMsg::ByOwner {
            owner,
            start_after,
            limit,
            order,
        } => {
            let owner = deps.api.addr_validate(&owner)?;
            let responses: StdResult<Vec<GetPositionResponse>> =
                Position::list_by_owner(deps.storage, &owner, start_after, limit, order)?
                    .iter()
                    .map(|(id, position)| {
                        let position_with_fee =
                            position.clone().apply_fee(state.global_fee_index)?;

                        Ok(GetPositionResponse {
                            id: *id,
                            owner: position.owner.to_string(),
                            collateral: position.collateral,
                            debt: position.debt,
                            pending_fee: position_with_fee.debt.checked_sub(position.debt)?,
                        })
                    })
                    .collect();

            to_binary(&ListPositionResponse(responses?))
        }
    }
}
