use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: HumanAddr,
    pub gov_token_addr: HumanAddr,
    pub gov_token_hash: String,
    pub total_weight: u64,
    pub minting_schedule: Schedule,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardContract {
    pub weight: u64,
    pub last_update_block: u64,
    // pub eligible_for: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Copy)]
pub struct ScheduleUnit {
    pub end_block: u64,
    pub mint_per_block: u128,
}

pub type Schedule = Vec<ScheduleUnit>;

pub fn sort_schedule(s: &mut Schedule) {
    s.sort_by(|&s1, &s2| s1.end_block.cmp(&s2.end_block))
}
