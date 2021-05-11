use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    title: String,
    description: String,
    additional: Option<String>,
    duration: Option<u64>,     // TODO: Might want to change this later
    quorum: Option<u8>,        // X/100% (percentage)
    min_threshold: Option<u8>, // X/100% (percentage)
    choices: Vec<String>,
    author: HumanAddr,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Vote {
        choice: u8, // Arbitrary id that is given by the contract
    },
    UpdateVotingPower {
        voter: HumanAddr,
        new_power: Uint128,
    },
    Finalize {},
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Choices {},
    Tally {}, // Only when poll is finished
    HasVoted { voter: HumanAddr },
    Voters {}, // Only when poll is finished
}
