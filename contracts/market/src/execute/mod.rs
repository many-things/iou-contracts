pub mod collateral;
pub mod debt;
pub mod position;

use cosmwasm_std::{Env, MessageInfo};
use noi_alias::{DepsMut, Response};
use noi_interface::market::ConfigMsg;

use crate::{
    state::{Config, State},
    ContractError,
};

pub use crate::execute::collateral::{lock, unlock};
pub use crate::execute::debt::{borrow, repay};
pub use crate::execute::position::{close, liquidate, open};

pub fn config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ConfigMsg,
) -> Result<Response, ContractError> {
    let mut config = Config::load(deps.storage)?;
    if info.sender != config.core {
        return Err(ContractError::Unauthorized {});
    }

    State::load(deps.storage)?
        .update_fee(&env.block.time, config.fee_multiplier)?
        .save(deps.storage)?;

    match msg {
        ConfigMsg::ManageRoles {
            core,
            oracle,
            liquidator,
        } => {
            if let Some(core) = core {
                config.core = deps.api.addr_validate(&core)?;
            }
            if let Some(oracle) = oracle {
                config.oracle = deps.api.addr_validate(&oracle)?;
            }
            if let Some(liquidator) = liquidator {
                config.liquidator = deps.api.addr_validate(&liquidator)?;
            }
        }
        ConfigMsg::AdjustLTV { borrow_ltv } => {
            if let Some(borrow_ltv) = borrow_ltv {
                config.borrow_ltv = borrow_ltv;
            }
        }
        ConfigMsg::AdjustFeeMultiplier(fee_multiplier) => {
            config.fee_multiplier = fee_multiplier;
        }
    }

    config.save(deps.storage)?;

    Ok(Response::new())
}
