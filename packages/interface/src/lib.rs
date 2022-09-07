pub mod helpers;

pub mod core;
pub mod market;
pub mod oracle;
pub mod treasury;

use cosmwasm_std::Order;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Settings for pagination
pub const MAX_LIMIT: u32 = 30;
pub const DEFAULT_LIMIT: u32 = 10;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RangeOrder {
    Asc,
    Desc,
}

impl From<Order> for RangeOrder {
    fn from(order: Order) -> Self {
        match order {
            Order::Ascending => Self::Asc,
            Order::Descending => Self::Desc,
        }
    }
}

impl From<RangeOrder> for Order {
    fn from(order: RangeOrder) -> Self {
        match order {
            RangeOrder::Asc => Order::Ascending,
            RangeOrder::Desc => Order::Descending,
        }
    }
}
