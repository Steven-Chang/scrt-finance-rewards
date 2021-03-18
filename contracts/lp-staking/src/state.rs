use cosmwasm_std::HumanAddr;
use scrt_finance::lp_staking_types::SecretContract;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Config {
    pub admin: HumanAddr,
    pub reward_token: SecretContract,
    pub inc_token: SecretContract,
    pub master: SecretContract,
    pub viewing_key: String,
    pub prng_seed: Vec<u8>,
    pub is_stopped: bool,
    pub own_addr: HumanAddr,
}
