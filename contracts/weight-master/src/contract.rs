use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, Uint128, WasmMsg,
};

use crate::msg::CallbackMsg::NotifyAllocation;
use crate::msg::{HandleAnswer, HandleMsg, InitMsg, QueryMsg, WeightInfo};
use crate::state::{config, config_read, RewardContract, Schedule, State};
use secret_toolkit::snip20;
use secret_toolkit::storage::TypedStoreMut;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        admin: env.message.sender,
        gov_token_addr: msg.gov_token_addr,
        gov_token_hash: msg.gov_token_hash,
        total_weight: 0,
        minting_schedule: msg.minting_schedule,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        _ => Ok(HandleResponse {
            messages: vec![],
            log: vec![],
            data: None,
        }),
    }
}

fn set_weights<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    weights: Vec<WeightInfo>,
) -> StdResult<HandleResponse> {
    let mut state = config_read(&deps.storage).load()?;

    let mut messages = vec![];
    let mut logs = vec![];
    let mut new_weight_counter = 0;
    let mut old_weight_counter = 0;

    // Update reward contracts one by one
    for to_update in weights {
        let mut rs = TypedStoreMut::attach(&mut deps.storage);
        let mut reward_contract =
            rs.load(to_update.address.clone().0.as_bytes())
                .unwrap_or(RewardContract {
                    weight: 0,
                    last_update_block: env.block.height.clone(),
                });

        if reward_contract.last_update_block < env.block.height {
            // Calc amount to mint for this reward contract and push to messages
            let rewards = get_spy_rewards(
                env.block.height,
                state.total_weight,
                &state.minting_schedule,
                reward_contract.clone(),
            );
            messages.push(snip20::mint_msg(
                to_update.address.clone(),
                Uint128(rewards),
                None,
                1,
                state.gov_token_hash.clone(),
                state.gov_token_addr.clone(),
            )?);

            // Notify to the spy contract on the new allocation
            messages.push(
                WasmMsg::Execute {
                    contract_addr: to_update.address.clone(),
                    callback_code_hash: to_update.hash,
                    msg: to_binary(&NotifyAllocation {
                        amount: Uint128(rewards),
                        hook: None,
                    })?,
                    send: vec![],
                }
                .into(),
            );
        }

        let old_weight = reward_contract.weight;
        let new_weight = to_update.weight;

        // Set new weight and update total counter
        reward_contract.weight = new_weight;
        reward_contract.last_update_block = env.block.height;
        rs.store(to_update.address.0.as_bytes(), &reward_contract)?;

        // Update counters to batch update after the loop
        new_weight_counter += new_weight;
        old_weight_counter += old_weight;

        logs.push(log("weight_update", to_update.address.0))
    }

    state.total_weight = state.total_weight - old_weight_counter + new_weight_counter;
    config(&mut deps.storage).save(&state);

    Ok(HandleResponse {
        messages,
        log: logs,
        data: Some(to_binary(&HandleAnswer::Success)?),
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        _ => Ok(Binary(vec![])),
    }
}

fn get_spy_rewards(
    current_block: u64,
    total_weight: u64,
    schedule: &Schedule,
    reward_contract: RewardContract,
) -> u128 {
    let mut last_update_block = reward_contract.last_update_block;

    let mut multiplier = 0;
    // Going serially assuming that schedule is not a big vector
    for u in schedule.clone() {
        if last_update_block < u.end_block {
            if current_block > u.end_block {
                multiplier += (u.end_block - last_update_block) as u128 * u.mint_per_block;
                last_update_block = u.end_block;
            } else {
                multiplier += (current_block - last_update_block) as u128 * u.mint_per_block;
                // last_update_block = current_block;
                break; // No need to go further up the schedule
            }
        }
    }

    (multiplier * reward_contract.weight as u128) / total_weight as u128
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};
}
