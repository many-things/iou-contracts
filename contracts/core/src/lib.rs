pub mod contract;
mod error;
pub mod execute;
pub mod state;

pub const CONTRACT_NAME: &str = "crates.io:noi-core";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPLY_ID_TREAURY_CREATION: u64 = 0;
pub const REPLY_ID_LIQUIDATOR_CREATION: u64 = 1;
pub const REPLY_ID_MARKET_CREATION: u64 = 2;

pub use crate::error::ContractError;
