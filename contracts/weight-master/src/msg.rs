use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use scrt_finance::master_types::Schedule;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub gov_token_addr: HumanAddr,
    pub gov_token_hash: String,
    pub minting_schedule: Schedule,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Admin {},
    GovToken {},
    Schedule {},
    SpyWeight { addr: HumanAddr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    Admin {
        address: HumanAddr,
    },
    GovToken {
        token_addr: HumanAddr,
        token_hash: String,
    },
    Schedule {
        schedule: Schedule,
    },
    SpyWeight {
        weight: u64,
    },
}
