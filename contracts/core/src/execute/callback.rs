use cosmwasm_std::{Env, MessageInfo};
use noi_alias::{DepsMut, Response};
use noi_interface::core::CallbackMsg;

use crate::ContractError;

pub fn handle_callback_msg(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: CallbackMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}
