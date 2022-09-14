use cosmwasm_std::{to_binary, Env, MessageInfo, WasmMsg};
use noi_alias::{DepsMut, Response, SubMsg};
use noi_interface::{
    core::{MarketCreateMsg, MarketMsg},
    market,
};

use crate::{
    state::{Config, Market},
    ContractError, REPLY_ID_MARKET_CREATION,
};

pub fn handle_market_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MarketMsg,
) -> Result<Response, ContractError> {
    match msg {
        MarketMsg::Create(msg) => create(deps, env, info, msg),
        MarketMsg::Config { market, msg } => config(deps, env, info, market, msg),
    }
}

fn create(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MarketCreateMsg,
) -> Result<Response, ContractError> {
    let config = Config::load(deps.storage)?;
    if config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let market_contract_label = format!("noi-market::{}", msg.name);
    let market_init_msg = market::InstantiateMsg {
        name: msg.name,

        core: env.contract.address.to_string(),
        oracle: msg.oracle,
        liquidator: config.liquidator.into_string(),

        debt_asset: config.denom,
        collateral_asset: msg.collateral,

        borrow_ltv: msg.borrow_ltv,
        fee_base: msg.fee_base,
        fee_multiplier: msg.fee_multiplier,
    };

    let wasm_init_msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: msg.code_id,
        msg: to_binary(&market_init_msg)?,
        funds: vec![],
        label: market_contract_label,
    };

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        wasm_init_msg,
        REPLY_ID_MARKET_CREATION,
    )))
}

fn config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    market: String,
    msg: market::ConfigMsg,
) -> Result<Response, ContractError> {
    let config = Config::load(deps.storage)?;
    if config.gov != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    match msg {
        market::ConfigMsg::ChangeName(ref new_name) => {
            let addr = deps.api.addr_validate(&market)?;
            let mut market = Market::load(deps.storage, &addr)?;

            Market::raw_idx_name().remove(deps.storage, &market.name);
            Market::raw_idx_name().save(deps.storage, new_name, &addr)?;

            market.name = new_name.clone();
            market.save(deps.storage, &addr)?;
        }
        _ => {}
    }

    let market_config_msg = market::ExecuteMsg::Config(msg);
    let wasm_execute_msg = WasmMsg::Execute {
        contract_addr: market,
        msg: to_binary(&market_config_msg)?,
        funds: vec![],
    };

    Ok(Response::new().add_message(wasm_execute_msg))
}
