use cosmwasm_std::HumanAddr;
use scrt_finance::types::SecretContract;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Config {
    pub admin: HumanAddr,
    pub beneficiary: HumanAddr,
    pub sefi: SecretContract,
    pub master: SecretContract,
    pub viewing_key: String,
    pub own_addr: HumanAddr,
}
