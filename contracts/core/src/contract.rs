#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coins, to_binary, Addr, Binary, Env, MessageInfo, Reply, WasmMsg};

use noi_alias::{Deps, DepsMut, Response, SubMsg};
use noi_interface::{
    core::{ExecuteMsg, InstantiateMsg, LiquidatorInfo, QueryMsg, TreasuryInfo},
    helpers::NoiMarket,
    treasury,
};

use crate::{
    error::ContractError,
    state::{Config, Market},
    CONTRACT_NAME, CONTRACT_VERSION, REPLY_ID_LIQUIDATOR_CREATION, REPLY_ID_MARKET_CREATION,
    REPLY_ID_TREAURY_CREATION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mut config = Config {
        gov: deps.api.addr_validate(&msg.gov)?,
        denom: msg.denom.clone(),
        treasury: Addr::unchecked(""),
        liquidator: Addr::unchecked(""),
    };

    let mut resp = Response::new();

    match msg.treasury {
        TreasuryInfo::Create {
            code_id,
            fee,
            fee_asset,
        } => {
            let received = cw_utils::must_pay(&info, &fee_asset)?;
            if received != fee {
                return Err(ContractError::InsufficientTokenCreationFee {});
            }

            resp = resp.add_submessage(SubMsg::reply_on_success(
                WasmMsg::Instantiate {
                    admin: Some(env.contract.address.to_string()),
                    code_id,
                    msg: to_binary(&treasury::InstantiateMsg {
                        denom: msg.denom,
                        core: env.contract.address.to_string(),
                    })?,
                    funds: coins(received.u128(), fee_asset),
                    label: "noi-treasury".to_string(),
                },
                REPLY_ID_TREAURY_CREATION,
            ));
        }
        TreasuryInfo::Reuse(treasury) => {
            config.treasury = deps.api.addr_validate(&treasury)?;
        }
    }

    match msg.liquidator {
        LiquidatorInfo::Create { code_id } => {
            resp = resp.add_submessage(SubMsg::reply_on_success(
                WasmMsg::Instantiate {
                    admin: Some(env.contract.address.to_string()),
                    code_id,
                    msg: to_binary(&())?,
                    funds: vec![],
                    label: "noi-liquidator".to_string(),
                },
                REPLY_ID_LIQUIDATOR_CREATION,
            ));
        }
        LiquidatorInfo::Reuse(liquidator) => {
            config.liquidator = deps.api.addr_validate(&liquidator)?;
        }
    }

    config.save(deps.storage)?;

    Ok(resp)
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
        Config(msg) => execute::config(deps, env, info, msg),
        Market(msg) => execute::handle_market_msg(deps, env, info, msg),
        Callback(msg) => execute::handle_callback_msg(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_TREAURY_CREATION => {
            let resp = cw_utils::parse_reply_instantiate_data(msg)?;

            let mut config = Config::load(deps.storage)?;
            config.treasury = deps.api.addr_validate(&resp.contract_address)?;
            config.save(deps.storage)?;

            Ok(Response::default())
        }
        REPLY_ID_LIQUIDATOR_CREATION => {
            let resp = cw_utils::parse_reply_instantiate_data(msg)?;

            let mut config = Config::load(deps.storage)?;
            config.liquidator = deps.api.addr_validate(&resp.contract_address)?;
            config.save(deps.storage)?;

            Ok(Response::default())
        }
        REPLY_ID_MARKET_CREATION => {
            let resp = cw_utils::parse_reply_instantiate_data(msg)?;

            let market = NoiMarket(deps.api.addr_validate(&resp.contract_address)?);
            let market_config = market.get_config(&deps.querier)?;

            Market {
                name: market_config.name,
                collateral: market_config.collateral_asset,
            }
            .save(deps.storage, &market.addr())?;

            Ok(Response::default())
        }
        _ => Err(ContractError::InvalidReplyId {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
