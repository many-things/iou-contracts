use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::treasury::{ConfigResponse, ExecuteMsg, QueryMsg};

/// NoiTreasury is a wrapper around Addr that provides a lot of helpers
/// for working with this. Rename it to your contract name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NoiTreasury(pub Addr);

impl NoiTreasury {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>, CM>(&self, msg: T) -> StdResult<CosmosMsg<CM>> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn config<CQ>(&self, querier: &QuerierWrapper<CQ>) -> StdResult<ConfigResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::Config {};
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: ConfigResponse = querier.query(&query)?;
        Ok(res)
    }
}
