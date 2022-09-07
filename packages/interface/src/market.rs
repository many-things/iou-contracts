use cosmwasm_std::{Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    // addr
    pub core: String,
    pub oracle: String,
    pub liquidator: String,

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
        liquidator: Option<String>,
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
pub enum ListPosition {
    Default {},
    ByAccount {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetStatus {},
    GetPosition {},
    ListPosition(ListPosition),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetConfigResponse {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetStatusResponse {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetPositionResponse {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListPositionResponse {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
