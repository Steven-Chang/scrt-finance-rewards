use crate::state::Schedule;
use cosmwasm_std::{Binary, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub gov_token_addr: HumanAddr,
    pub gov_token_hash: String,
    pub minting_schedule: Schedule,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    // Callbacks
    // MintFor { address: HumanAddr, amount: Uint128 },
    MintAllocation {},

    // Admin commands
    SetWeights { weights: Vec<WeightInfo> },
    SetSchedule { schedule: Schedule },
    SetGovToken { addr: HumanAddr, hash: String },
    ChangeAdmin { addr: HumanAddr },
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
    GetAllocation {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CallbackMsg {
    NotifyAllocation {
        amount: Uint128,
        hook: Option<Binary>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WeightInfo {
    pub address: HumanAddr,
    pub hash: String,
    pub weight: u64,
}

// // We define a custom struct for each query response
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct CountResponse {
//     pub count: i32,
// }
