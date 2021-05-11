use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128, WasmMsg,
};

use crate::state::{config, config_read, State};
use scrt_finance::lp_staking_msg::LPStakingHandleMsg;
use scrt_finance::master_msg::{MasterHandleAnswer, MasterInitMsg, MasterQueryMsg};
use scrt_finance::master_msg::{MasterHandleMsg, MasterQueryAnswer};
use scrt_finance::types::{sort_schedule, Schedule, SpySettings, WeightInfo};
use secret_toolkit::snip20;
use secret_toolkit::storage::{TypedStore, TypedStoreMut};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: MasterInitMsg,
) -> StdResult<InitResponse> {
    unimplemented!()
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: MasterHandleMsg,
) -> StdResult<HandleResponse> {
    unimplemented!()
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: MasterQueryMsg,
) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};
}
