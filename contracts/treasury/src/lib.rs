use osmo_bindings_tokenfactory::{OsmosisMsg, OsmosisQuery};

pub use crate::error::ContractError;

// Type aliases
pub type Response = cosmwasm_std::Response<OsmosisMsg>;
pub type SubMsg = cosmwasm_std::SubMsg<OsmosisMsg>;
pub type CosmosMsg = cosmwasm_std::CosmosMsg<OsmosisMsg>;
pub type Deps<'a> = cosmwasm_std::Deps<'a, OsmosisQuery>;
pub type DepsMut<'a> = cosmwasm_std::DepsMut<'a, OsmosisMsg>;
pub type QuerierWrapper<'a> = cosmwasm_std::QuerierWrapper<'a, OsmosisQuery>;

pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;
