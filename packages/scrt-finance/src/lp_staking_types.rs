use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// RewardPool is a struct that keeps track of rewards and lockups
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct RewardPool {
    pub residue: u128,
    pub inc_token_supply: u128,
    pub acc_reward_per_share: u128,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, JsonSchema)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub locked: u128,
    pub debt: u128,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecretContract {
    pub address: HumanAddr,
    pub contract_hash: String,
}
