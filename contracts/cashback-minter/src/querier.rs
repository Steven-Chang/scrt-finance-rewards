use crate::asset::AssetInfo;
use cosmwasm_std::{to_binary, HumanAddr, Querier, QueryRequest, StdResult, Uint128, WasmQuery};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PairQueryMsg {
    Pair {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// This is copied from SecretSwap
pub struct PairInfo {
    pub asset_infos: [AssetInfo; 2],
    pub contract_addr: HumanAddr,
    pub liquidity_token: HumanAddr,
    pub token_code_hash: String,
    pub asset0_volume: Uint128,
    pub asset1_volume: Uint128,
    pub factory: Factory,
}

pub fn query_pair<Q: Querier>(
    querier: &Q,
    pair: HumanAddr,
    pair_hash: String,
) -> StdResult<PairInfo> {
    let pair_info: PairInfo = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pair,
        callback_code_hash: pair_hash,
        msg: to_binary(&PairQueryMsg::Pair {})?,
    }))?;

    Ok(pair_info)
}
