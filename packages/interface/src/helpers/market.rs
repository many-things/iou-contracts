use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Empty, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::market::{
    ExecuteMsg, GetConfigResponse, GetPositionResponse, GetStatusResponse, ListPosition,
    ListPositionResponse, QueryMsg,
};

/// NoiMarket is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NoiMarket(pub Addr);

impl NoiMarket {
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

    pub fn get_config<CQ>(&self, querier: &QuerierWrapper<CQ>) -> StdResult<GetConfigResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::GetConfig {};

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn get_status<CQ>(&self, querier: &QuerierWrapper<CQ>) -> StdResult<GetStatusResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::GetStatus {};

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn get_position<CQ>(&self, querier: &QuerierWrapper<CQ>) -> StdResult<GetPositionResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::GetPosition {};

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn list_position<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        opt: ListPosition,
    ) -> StdResult<ListPositionResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::ListPosition(opt);

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }
}
