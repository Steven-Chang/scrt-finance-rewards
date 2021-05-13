use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128, WasmMsg,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{
    ChoiceIdMap, Tally, CHOICE_ID_MAP_KEY, CONFIG_KEY, METADATA_KEY, OWNER_KEY, TALLY_KEY,
};
use secret_toolkit::snip20;
use secret_toolkit::storage::{TypedStore, TypedStoreMut};
use std::collections::HashMap;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let owner = env.message.sender;
    TypedStoreMut::attach(&mut deps.storage).store(OWNER_KEY, &owner)?;

    TypedStoreMut::attach(&mut deps.storage).store(METADATA_KEY, &msg.metadata)?;
    TypedStoreMut::attach(&mut deps.storage).store(CONFIG_KEY, &msg.config)?;

    if msg.choices.len() > (u8::MAX - 1) as usize {
        return Err(StdError::generic_err(format!(
            "the number of choices for a poll cannot exceed {}",
            u8::MAX - 1
        )));
    }

    // Creating a mapping between a choice's text and it's ID for convenience
    let mut i = 0;
    let choice_id_map: ChoiceIdMap = msg
        .choices
        .iter()
        .map(|c| {
            i += 1;
            (i, c.clone())
        })
        .collect();
    TypedStoreMut::attach(&mut deps.storage).store(CHOICE_ID_MAP_KEY, &choice_id_map)?;

    let mut tally: Tally = HashMap::new();
    for choice in choice_id_map {
        tally.insert(choice.0, 0);
    }
    TypedStoreMut::attach(&mut deps.storage).store(TALLY_KEY, &tally)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    unimplemented!()
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};
}
