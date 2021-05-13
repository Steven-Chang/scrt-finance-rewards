use cosmwasm_std::HumanAddr;
use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const OWNER_KEY: &[u8] = b"owner";
pub const CHOICE_ID_MAP_KEY: &[u8] = b"choiceidmap";
pub const TALLY_KEY: &[u8] = b"tally";
pub const METADATA_KEY: &[u8] = b"metadata";
pub const CONFIG_KEY: &[u8] = b"config";

pub type ChoiceIdMap = Vec<(u8, String)>;
pub type Tally = HashMap<u8, u128>;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PollConfig {
    pub duration: Option<u64>,     // TODO: Might want to change this later
    pub quorum: Option<u8>,        // X/100% (percentage)
    pub min_threshold: Option<u8>, // X/100% (percentage)
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PollMetadata {
    pub title: String,
    pub description: String,
    pub additional: Option<String>,
    pub author: HumanAddr,
}
