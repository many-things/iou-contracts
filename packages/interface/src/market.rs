use cosmwasm_std::{Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::RangeOrder;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    // addr
    pub core: String,
    pub oracle: String,

    // denom
    pub debt_asset: String,
    pub collateral_asset: String,

    // option
    pub borrow_ltv: Decimal,
    pub fee_base: Decimal,
    pub fee_multiplier: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConfigMsg {
    ManageRoles {
        core: Option<String>,
        oracle: Option<String>,
    },
    AdjustLTV {
        borrow_ltv: Option<Decimal>,
    },
    AdjustFeeMultiplier(Decimal),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Config(ConfigMsg),
    // position
    Open {},
    Close { position_id: u64 },
    Liquidate { position_id: u64 }, // Internal method - core && liquidator only
    // collateral
    Lock { position_id: u64 },
    Unlock { position_id: u64, amount: Uint128 },
    // debt
    Borrow { position_id: u64, amount: Uint128 },
    Repay { position_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ListPositionMsg {
    Default {
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },
    ByOwner {
        owner: String,
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetState {},
    GetPosition { position_id: u64 },
    ListPosition(ListPositionMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetConfigResponse {
    pub core: String,
    pub oracle: String,

    pub debt_asset: String,
    pub collateral_asset: String,

    pub borrow_ltv: Decimal,
    pub fee_multiplier: Decimal,
    pub fee_apy: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetStateResponse {
    pub total_debt: Uint128,
    pub total_collateral: Uint128,
    pub fee_updated_at: u64,
    pub global_fee_index: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetPositionResponse {
    pub id: u64,
    pub owner: String,
    pub collateral: Uint128,
    pub debt: Uint128,
    pub pending_fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListPositionResponse(pub Vec<GetPositionResponse>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
