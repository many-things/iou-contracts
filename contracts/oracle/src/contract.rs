#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Env, MessageInfo, StdResult};
use noi_alias::{Deps, DepsMut, Response};
use noi_interface::oracle::{
    ExecuteMsg, GetConfigResponse, GetRateResponse, InstantiateMsg, QueryMsg,
};
use osmo_bindings::OsmosisQuerier;

use crate::{
    error::ContractError,
    state::{Config, CONFIG},
};

const CONTRACT_NAME: &str = "crates.io:noi-core";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.time_range > (24 * 60 * 60) * 2 {
        return Err(ContractError::InvalidTimeRange {});
    }

    CONFIG.save(
        deps.storage,
        &Config {
            gov: deps.api.addr_validate(&msg.gov)?,
            pool_id: msg.pool_id,
            quote_asset: msg.quote_asset,
            base_asset: msg.base_asset,
            time_range: msg.time_range,
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.gov {
        return Err(ContractError::Unauthorized {});
    }

    match msg {
        UpdateGov(new_gov) => {
            config.gov = deps.api.addr_validate(&new_gov)?;
        }
        UpdatePool(new_pool) => {
            config.pool_id = new_pool.pool_id;
            config.quote_asset = new_pool.quote_asset;
            config.base_asset = new_pool.base_asset;
        }
        AdjustTimeRange(new_time_range) => {
            config.time_range = new_time_range;
        }
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    let config = CONFIG.load(deps.storage)?;

    match msg {
        GetConfig {} => to_binary(&GetConfigResponse {
            gov: config.gov.into_string(),
            pool_id: config.pool_id,
            quote_asset: config.quote_asset,
            base_asset: config.base_asset,
            time_range: config.time_range,
        }),
        GetRate {} => {
            let querier = OsmosisQuerier::new(&deps.querier);

            let rate = querier
                .arithmetic_twap_to_now(
                    config.pool_id,
                    config.quote_asset.clone(),
                    config.base_asset.clone(),
                    (env.block.time.seconds() - config.time_range) as i64,
                )?
                .twap;

            to_binary(&GetRateResponse {
                quote_asset: config.quote_asset,
                base_asset: config.base_asset,
                rate,
            })
        }
    }
}

#[cfg(test)]
mod tests {}
