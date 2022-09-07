use cosmwasm_std::{Decimal, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] cosmwasm_std::OverflowError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    CheckedFromRatioError(#[from] cosmwasm_std::CheckedFromRatioError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("insufficient collateral. (demand {demand:?}, actual {actual:?})")]
    InsufficientCollateral { demand: Decimal, actual: Decimal },

    #[error("unlock capacity overflowed")]
    OverflowUnlockCapacity {},

    #[error("borrow capacity overflowed")]
    OverflowBorrowCapacity {},

    #[error("repay amount exceeds actual demand")]
    Overpaid {},

    #[error("not liquidated")]
    NotLiquidated {},

    #[error("not liquidatable")]
    NotLiquidatable {},

    #[error("oversized request. size: {size:?}, max: {max:?}")]
    OversizedRequest { size: u64, max: u64 },
}
