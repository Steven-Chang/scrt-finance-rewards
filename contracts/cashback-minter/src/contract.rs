use cosmwasm_std::{
    Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, ReadonlyStorage,
    StdError, StdResult, Storage, Uint128,
};

use crate::asset::{Asset, AssetInfo};
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{Pair, KEY_ADMIN, KEY_CSHBK, KEY_SSCRT, PREFIX_PAIRED_TOKENS};
use cosmwasm_storage::PrefixedStorage;
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
        .store(KEY_SSCRT, &msg.cashback)?;
    TypedStoreMut::<HumanAddr, S>::attach(&mut deps.storage)
        .store(KEY_ADMIN, &env.message.sender)?;

    set_pairs_to_storage(deps, msg.pairs)?;

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
        HandleMsg::AddPairs { pairs } => add_pairs(deps, env, pairs),
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
    let amount = get_eligibility(deps, asset_in, asset_out)?;

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
        data: None,
    })
}

fn add_pairs<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pairs: Vec<Pair>,
) -> StdResult<HandleResponse> {
    verify_admin(deps, env)?;
    set_pairs_to_storage(deps, pairs)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: None,
    })
}

fn remove_pairs<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pairs: Vec<Pair>,
) -> StdResult<HandleResponse> {
    verify_admin(deps, env)?;
    remove_pairs_from_storage(deps, pairs)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: None,
    })
}

fn set_admin<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    address: HumanAddr,
) -> StdResult<HandleResponse> {
    verify_admin(deps, env)?;

    TypedStoreMut::<HumanAddr, S>::attach(&mut deps.storage).store(KEY_ADMIN, &address)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: None,
    })
}

// Helper functions

fn get_eligibility<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    asset_in: Asset,
    asset_out: Asset,
) -> StdResult<u128> {
    let scrt: Asset;
    let paired: Asset;

    if is_scrt(deps, asset_in.clone())? {
        scrt = asset_in;
        paired = asset_out;
    } else if is_scrt(deps, asset_out.clone())? {
        scrt = asset_out;
        paired = asset_in;
    } else {
        return Ok(0);
    }

    let paired_addr: HumanAddr = match paired.info {
        AssetInfo::Token { contract_addr, .. } => contract_addr,
        AssetInfo::NativeToken { .. } => return Ok(scrt.amount.0), // If paired is native => this is the SCRT<>sSCRT pair
    };
    let is_stored =
        PrefixedStorage::new(PREFIX_PAIRED_TOKENS, &mut deps.storage).get(paired_addr.0.as_bytes());
    if is_stored.is_none() {
        // If stored => eligible
        return Ok(0);
    }

    Ok(scrt.amount.0) // Eligibility amount is arbitrarily set to the SCRT value of the swap
}

fn is_scrt<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    asset: Asset,
) -> StdResult<bool> {
    match asset.info {
        AssetInfo::Token { contract_addr, .. } => {
            let sscrt = TypedStore::<HumanAddr, S>::attach(&deps.storage).load(KEY_SSCRT)?;
            Ok(contract_addr == sscrt)
        }
        AssetInfo::NativeToken { denom } => Ok(denom.to_lowercase() == "uscrt".to_string()),
    }
}

fn set_pairs_to_storage<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    pairs: Vec<Pair>,
) -> StdResult<()> {
    let sscrt_addr = TypedStoreMut::<HumanAddr, S>::attach(&mut deps.storage).load(KEY_SSCRT)?;

    let mut supported_tokens = PrefixedStorage::new(PREFIX_PAIRED_TOKENS, &mut deps.storage);
    for pair in pairs {
        // Get the token that is not sSCRT
        let token;
        if pair.asset_0 == sscrt_addr {
            token = pair.asset_1;
        } else if pair.asset_1 == sscrt_addr {
            token = pair.asset_0;
        } else {
            return Err(StdError::generic_err(
                "invalid pair! One of the sides has to be sSCRT",
            ));
        }
        // Value is irrelevant, just marking that token as supported
        supported_tokens.set(token.0.as_bytes(), &[1]);
    }

    Ok(())
}

fn remove_pairs_from_storage<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    pairs: Vec<Pair>,
) -> StdResult<()> {
    let sscrt_addr = TypedStoreMut::<HumanAddr, S>::attach(&mut deps.storage).load(KEY_SSCRT)?;

    let mut supported_tokens = PrefixedStorage::new(PREFIX_PAIRED_TOKENS, &mut deps.storage);
    for pair in pairs {
        // Get the token that is not sSCRT
        let token;
        if pair.asset_0 == sscrt_addr {
            token = pair.asset_1;
        } else if pair.asset_1 == sscrt_addr {
            token = pair.asset_0;
        } else {
            return Err(StdError::generic_err(
                "invalid pair! One of the sides has to be sSCRT",
            ));
        }
        supported_tokens.remove(token.0.as_bytes());
    }

    Ok(())
}

fn verify_admin<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<()> {
    let admin: HumanAddr = TypedStore::attach(&deps.storage).load(KEY_ADMIN)?;

    if admin == env.message.sender {
        return Ok(());
    }

    Err(StdError::generic_err("not an admin!"))
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pairs { .. } => unimplemented!(),
    }
}
