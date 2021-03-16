use crate::state::{SecretContract, TokenInfo};
use crate::viewing_key::ViewingKey;
use cosmwasm_std::{Binary, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub reward_token: SecretContract,
    pub inc_token: SecretContract,
    pub master: SecretContract,
    pub viewing_key: String,
    pub token_info: TokenInfo,
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
    ContractStatus {},
    RewardToken {},
    IncentivizedToken {},

    // Authenticated
    Rewards {
        address: HumanAddr,
        new_rewards: Uint128,
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
    ContractStatus {
        is_stopped: bool,
    },
    RewardToken {
        token: SecretContract,
    },
    IncentivizedToken {
        token: SecretContract,
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
