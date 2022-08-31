#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult, Uint128};
use osmo_bindings_tokenfactory::OsmosisMsg;

use noi_alias::Response;

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:noi-treasury";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let core = deps.api.addr_validate(msg.core.as_str())?;

    CONFIG.save(
        deps.storage,
        &Config {
            core,
            denom: msg.denom.clone(),
        },
    )?;

    Ok(Response::default().add_message(OsmosisMsg::CreateDenom {
        subdenom: msg.denom,
    }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // execution is privileged for core contract
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.core {
        return Err(ContractError::Unauthorized {});
    }

    let action = match msg {
        ExecuteMsg::Mint { to, amount } => OsmosisMsg::MintTokens {
            denom: config.denom,
            mint_to_address: to,
            amount,
        },
        ExecuteMsg::Burn {} => {
            let amount = cw_utils::must_pay(&info, &config.denom)?;

            OsmosisMsg::BurnTokens {
                denom: config.denom,
                amount,
                burn_from_address: env.contract.address.into_string(),
            }
        }
        ExecuteMsg::Migrate { new_admin } => OsmosisMsg::ChangeAdmin {
            denom: config.denom,
            new_admin_address: new_admin,
        },
    };

    Ok(Response::default().add_message(action))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => {
            let config = CONFIG.load(deps.storage)?;

            to_binary(&ConfigResponse {
                denom: config.denom,
                core: config.core.into_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {}
