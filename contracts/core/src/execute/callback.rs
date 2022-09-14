use cosmwasm_std::{Env, MessageInfo};
use noi_alias::{DepsMut, Response};
use noi_interface::core::CallbackMsg;

use crate::ContractError;

pub fn handle_callback_msg(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: CallbackMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}
