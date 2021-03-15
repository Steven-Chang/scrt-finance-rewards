use crate::state::SecretContract;
use crate::viewing_key::ViewingKey;
use cosmwasm_std::{Binary, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub reward_token: SecretContract,
    pub inc_token: SecretContract,
    pub master: SecretContract,
    pub deadline: u64,
    pub pool_claim_block: u64,
    pub viewing_key: String,
    pub prng_seed: Binary,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    Redeem { status: ResponseStatus },
    CreateViewingKey { key: ViewingKey },
    SetViewingKey { status: ResponseStatus },
    StopContract { status: ResponseStatus },
    ResumeContract { status: ResponseStatus },
    ChangeAdmin { status: ResponseStatus },
    SetDeadline { status: ResponseStatus },
    ClaimRewardPool { status: ResponseStatus },
    EmergencyRedeem { status: ResponseStatus },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    Deposit {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HookMsg {
    Deposit {
        from: HumanAddr,
        amount: Uint128,
    },
    Redeem {
        to: HumanAddr,
        amount: Option<Uint128>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveAnswer {
    Deposit { status: ResponseStatus },
    DepositRewards { status: ResponseStatus },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenInfo {},
    ClaimBlock {},
    ContractStatus {},
    RewardToken {},
    IncentivizedToken {},
    EndHeight {},
    RewardPoolBalance {},

    // Authenticated
    Rewards {
        address: HumanAddr,
        height: u64,
        key: String,
    },
    Deposit {
        address: HumanAddr,
        key: String,
    },
}

impl QueryMsg {
    pub fn get_validation_params(&self) -> (&HumanAddr, ViewingKey) {
        match self {
            QueryMsg::Rewards { address, key, .. } => (address, ViewingKey(key.clone())),
            QueryMsg::Deposit { address, key } => (address, ViewingKey(key.clone())),
            _ => panic!("This should never happen"),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    TokenInfo {
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: Option<Uint128>,
    },
    Rewards {
        rewards: Uint128,
    },
    Deposit {
        deposit: Uint128,
    },
    ClaimBlock {
        height: u64,
    },
    ContractStatus {
        is_stopped: bool,
    },
    RewardToken {
        token: SecretContract,
    },
    IncentivizedToken {
        token: SecretContract,
    },
    EndHeight {
        height: u64,
    },
    RewardPoolBalance {
        balance: Uint128,
    },

    QueryError {
        msg: String,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure,
}
