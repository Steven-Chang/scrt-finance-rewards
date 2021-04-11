use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128, WasmMsg,
};

use crate::msg::{HandleMsg, QueryMsg};
use crate::state::State;
use scrt_finance::lp_staking_msg::LPStakingHandleMsg;
use scrt_finance::master_msg::{MasterHandleAnswer, MasterInitMsg, MasterQueryMsg};
use scrt_finance::master_msg::{MasterHandleMsg, MasterQueryAnswer};
use scrt_finance::master_types::{sort_schedule, Schedule, SpySettings, WeightInfo};
use secret_toolkit::snip20;
use secret_toolkit::storage::{TypedStore, TypedStoreMut};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: MasterInitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::ReceiveSwapData {
            asset_in,
            asset_out,
            account,
        } => {}
    };
    unimplemented!()
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {}
}
