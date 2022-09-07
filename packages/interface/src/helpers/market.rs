use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Order, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    market::{
        ExecuteMsg, GetConfigResponse, GetPositionResponse, GetStateResponse, ListPositionMsg,
        ListPositionResponse, QueryMsg,
    },
    RangeOrder,
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

    pub fn get_status<CQ>(&self, querier: &QuerierWrapper<CQ>) -> StdResult<GetStateResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::GetState {};

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn get_position<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        position_id: u64,
    ) -> StdResult<GetPositionResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::GetPosition { position_id };

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
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<Order>,
    ) -> StdResult<ListPositionResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::ListPosition(ListPositionMsg::Default {
            start_after,
            limit,
            order: order.map(RangeOrder::from),
        });

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }

    pub fn list_position_by_owner<CQ>(
        &self,
        querier: &QuerierWrapper<CQ>,
        owner: Addr,
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<Order>,
    ) -> StdResult<ListPositionResponse>
    where
        CQ: CustomQuery,
    {
        let msg = QueryMsg::ListPosition(ListPositionMsg::ByOwner {
            owner: owner.into_string(),
            start_after,
            limit,
            order: order.map(RangeOrder::from),
        });

        querier.query(
            &WasmQuery::Smart {
                contract_addr: self.addr().into(),
                msg: to_binary(&msg)?,
            }
            .into(),
        )
    }
}
