use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::{ExecuteMsg, QueryMsg};

/// NoiCore is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NoiCore(pub Addr);

impl NoiCore {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<CM, T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg<CM>> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}
