use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WeightInfo {
    pub address: HumanAddr,
    pub hash: String,
    pub weight: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpySettings {
    pub weight: u64,
    pub last_update_block: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Copy)]
pub struct ScheduleUnit {
    pub end_block: u64,
    pub mint_per_block: Uint128,
}

pub type Schedule = Vec<ScheduleUnit>;

pub fn sort_schedule(s: &mut Schedule) {
    s.sort_by(|&s1, &s2| s1.end_block.cmp(&s2.end_block))
}
