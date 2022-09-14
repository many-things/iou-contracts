use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    ParseReplyError(#[from] cw_utils::ParseReplyError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("invalid reply id")]
    InvalidReplyId {},

    #[error("insufficient token creation fee")]
    InsufficientTokenCreationFee {},
}
