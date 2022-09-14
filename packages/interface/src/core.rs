use cosmwasm_std::{Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TreasuryInfo {
    Create {
        code_id: u64,
        fee: Uint128,
        fee_asset: String,
    }, // code_id
    Reuse(String), // address
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum LiquidatorInfo {
    Create { code_id: u64 }, // code_id
    Reuse(String),           // address
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub gov: String,
    pub denom: String,
    pub treasury: TreasuryInfo,
    pub liquidator: LiquidatorInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MarketCreateMsg {
    pub name: String,
    pub code_id: u64,

    pub oracle: String,
    pub collateral: String,

    pub borrow_ltv: Decimal,
    pub fee_base: Decimal,
    pub fee_multiplier: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConfigMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarketMsg {
    Create(MarketCreateMsg),
    Config {
        market: String,
        msg: crate::market::ConfigMsg,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CallbackMsg {
    Open {
        owner: String,
        position_id: u64,
    },
    Close {
        owner: String,
        position_id: u64,
    },
    Liquidation {
        owner: String,
        asset: String,
        collateral: Uint128,
        debt: Uint128,
    },

    Lock {
        owner: String,
        position_id: u64,
        amount: Uint128,
    },
    Unlock {
        owner: String,
        position_id: u64,
        amount: Uint128,
    },

    Borrow {
        owner: String,
        position_id: u64,
        amount: Uint128,
    },
    Repay {
        owner: String,
        position_id: u64,
        amount: Uint128,
    },
}

impl From<CallbackMsg> for ExecuteMsg {
    fn from(msg: CallbackMsg) -> Self {
        Self::Callback(msg)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Config(ConfigMsg),
    Market(MarketMsg),
    Callback(CallbackMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
