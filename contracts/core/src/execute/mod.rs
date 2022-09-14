mod callback;
mod market;

use cosmwasm_std::{Env, MessageInfo};
use noi_alias::{DepsMut, Response};
use noi_interface::core::ConfigMsg;

use crate::ContractError;

pub use crate::execute::callback::handle_callback_msg;
pub use crate::execute::market::handle_market_msg;

pub fn config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ConfigMsg,
) -> Result<Response, ContractError> {
    match msg {
        _ => Ok(Response::new()),
    }
}
