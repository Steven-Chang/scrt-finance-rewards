use crate::master_types::{Schedule, WeightInfo};
use cosmwasm_std::{Binary, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MasterHandleMsg {
    UpdateAllocation {
        spy_addr: HumanAddr,
        spy_hash: String,
        hook: Option<Binary>,
    },

    // Admin commands
    SetWeights {
        weights: Vec<WeightInfo>,
    },
    SetSchedule {
        schedule: Schedule,
    },
    SetGovToken {
        addr: HumanAddr,
        hash: String,
    },
    ChangeAdmin {
        addr: HumanAddr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LPStakingHandleMsg {
    Redeem {
        amount: Option<Uint128>,
    },
    CreateViewingKey {
        entropy: String,
        padding: Option<String>,
    },
    SetViewingKey {
        key: String,
        padding: Option<String>,
    },
    EmergencyRedeem {},

    // Registered commands
    Receive {
        sender: HumanAddr,
        from: HumanAddr,
        amount: Uint128,
        msg: Binary,
    },

    // Admin commands
    StopContract {},
    ResumeContract {},
    ChangeAdmin {
        address: HumanAddr,
    },

    // Master callbacks
    NotifyAllocation {
        amount: Uint128,
        hook: Option<Binary>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
// Duplicating because need a generic one in the master contract, but it has to be in each SPY's HandleMsg
pub enum CallbackMsg {
    NotifyAllocation {
        amount: Uint128,
        hook: Option<Binary>,
    },
}
