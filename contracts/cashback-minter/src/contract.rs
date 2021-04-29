use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    ReadonlyStorage, StdError, StdResult, Storage, Uint128,
};

use crate::asset::{Asset, AssetInfo};
use crate::msg::{HandleMsg, InitMsg, QueryAnswer, QueryMsg, ResponseStatus};
use crate::state::{
    remove_pairs_from_storage, set_pairs_to_storage, KEY_ADMIN, KEY_CSHBK, KEY_SSCRT,
    PREFIX_PAIRED_TOKENS,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use scrt_finance::types::SecretContract;
use secret_toolkit::snip20;
use secret_toolkit::storage::{TypedStore, TypedStoreMut};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    TypedStoreMut::<HumanAddr, S>::attach(&mut deps.storage)
        .store(KEY_SSCRT, &msg.sscrt_addr.clone())?;
    TypedStoreMut::<SecretContract, S>::attach(&mut deps.storage)
        .store(KEY_CSHBK, &msg.cashback)?;
    TypedStoreMut::<HumanAddr, S>::attach(&mut deps.storage)
        .store(KEY_ADMIN, &env.message.sender)?;

    if let Some(pairs) = msg.pairs {
        let pair_hash = msg.pair_contract_hash.ok_or_else(|| {
            return StdError::generic_err(
                "when providing pairs, you have to provide pair contract hash as well",
            );
        })?;

        set_pairs_to_storage(deps, pairs, pair_hash)?;
    }

    Ok(InitResponse::default())
}

// Handle functions
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
        } => receive_swap_data(deps, env, asset_in, asset_out, account),
        HandleMsg::AddPairs {
            pairs,
            pair_contract_hash,
        } => add_pairs(deps, env, pairs, pair_contract_hash),
        HandleMsg::RemovePairs { pairs } => remove_pairs(deps, env, pairs),
        HandleMsg::SetAdmin { address } => set_admin(deps, env, address),
    }
}

fn receive_swap_data<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    asset_in: Asset,
    asset_out: Asset,
    account: HumanAddr,
) -> StdResult<HandleResponse> {
    let amount = get_eligibility(deps, env, asset_in, asset_out)?;

    let mut messages = vec![];
    if amount > 0 {
        let cashback =
            TypedStore::<SecretContract, S>::attach(&mut deps.storage).load(KEY_CSHBK)?;
        messages.push(snip20::mint_msg(
            account,
            Uint128(amount),
            None,
            256,
            cashback.contract_hash,
            cashback.address,
        )?)
    }

    Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

fn add_pairs<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pairs: Vec<HumanAddr>,
    pair_contract_hash: String,
) -> StdResult<HandleResponse> {
    enforce_admin(deps, env)?;
    set_pairs_to_storage(deps, pairs, pair_contract_hash)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

fn remove_pairs<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pairs: Vec<HumanAddr>,
) -> StdResult<HandleResponse> {
    enforce_admin(deps, env)?;
    remove_pairs_from_storage(deps, pairs)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

fn set_admin<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    address: HumanAddr,
) -> StdResult<HandleResponse> {
    enforce_admin(deps, env)?;

    TypedStoreMut::<HumanAddr, S>::attach(&mut deps.storage).store(KEY_ADMIN, &address)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&ResponseStatus::Success)?),
    })
}

// Helper functions

fn get_eligibility<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    asset_in: Asset,
    asset_out: Asset,
) -> StdResult<u128> {
    let is_stored = PrefixedStorage::new(PREFIX_PAIRED_TOKENS, &mut deps.storage)
        .get(env.message.sender.0.as_bytes());
    if is_stored.is_none() {
        // If stored => eligible
        return Ok(0);
    }

    // Eligibility amount is arbitrarily set to the SCRT value of the swap
    if is_scrt(deps, asset_in.info.clone())? {
        Ok(asset_in.amount.0)
    } else if is_scrt(deps, asset_out.info.clone())? {
        Ok(asset_out.amount.0)
    } else {
        Ok(0)
    }
}

pub fn is_scrt<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    asset: AssetInfo,
) -> StdResult<bool> {
    match asset {
        AssetInfo::Token { contract_addr, .. } => {
            let sscrt = TypedStore::<HumanAddr, S>::attach(&deps.storage).load(KEY_SSCRT)?;
            Ok(contract_addr == sscrt)
        }
        AssetInfo::NativeToken { denom } => Ok(denom.to_lowercase() == "uscrt".to_string()),
    }
}

fn enforce_admin<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<()> {
    let admin: HumanAddr = TypedStore::attach(&deps.storage).load(KEY_ADMIN)?;

    if admin != env.message.sender {
        return Err(StdError::generic_err("not an admin!"));
    }

    Ok(())
}

// Query functions

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsSupported { pair } => query_is_eligible(deps, pair),
        QueryMsg::Cashback {} => query_cashback(deps),
        QueryMsg::Admin {} => query_admin(deps),
    }
}

fn query_is_eligible<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    pair: HumanAddr,
) -> StdResult<Binary> {
    let supported_tokens = ReadonlyPrefixedStorage::new(PREFIX_PAIRED_TOKENS, &deps.storage);
    let is_supported = supported_tokens.get(pair.0.as_bytes());

    to_binary(&QueryAnswer::IsSupported {
        is_supported: is_supported.is_some(),
    })
}

fn query_cashback<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
    let cashback: SecretContract = TypedStore::attach(&deps.storage).load(KEY_CSHBK)?;

    to_binary(&QueryAnswer::Cashback {
        address: cashback.address,
    })
}

fn query_admin<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
    let admin: HumanAddr = TypedStore::attach(&deps.storage).load(KEY_ADMIN)?;

    to_binary(&QueryAnswer::Admin { address: admin })
}
