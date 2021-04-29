use crate::asset::Asset;
use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use scrt_finance::types::SecretContract;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    pub sscrt_addr: HumanAddr,
    pub pairs: Option<Vec<HumanAddr>>,
    pub pair_contract_hash: Option<String>,
    pub cashback: SecretContract,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    ReceiveSwapData {
        asset_in: Asset,
        asset_out: Asset,
        account: HumanAddr,
    },

    // Admin
    AddPairs {
        pairs: Vec<HumanAddr>,
        pair_contract_hash: String,
    },
    RemovePairs {
        pairs: Vec<HumanAddr>,
    },
    SetAdmin {
        address: HumanAddr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    IsSupported { pair: HumanAddr },
    Cashback {},
    Admin {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    IsSupported { is_supported: bool },
    Cashback { address: HumanAddr },
    Admin { address: HumanAddr },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure,
}
