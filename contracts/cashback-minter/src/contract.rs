use cosmwasm_std::{
    Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, ReadonlyStorage,
    StdResult, Storage, Uint128,
};

use crate::asset::{Asset, AssetInfo};
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{KEY_CSHBK, KEY_SSCRT, PREFIX_PAIRED_TOKENS};
use cosmwasm_storage::PrefixedStorage;
use scrt_finance::types::SecretContract;
use secret_toolkit::snip20;
use secret_toolkit::storage::{TypedStore, TypedStoreMut};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    TypedStoreMut::<HumanAddr, S>::attach(&mut deps.storage).store(KEY_SSCRT, &msg.sscrt_addr)?;
    TypedStoreMut::<SecretContract, S>::attach(&mut deps.storage)
        .store(KEY_SSCRT, &msg.cashback)?;

    let mut supported_tokens = PrefixedStorage::new(PREFIX_PAIRED_TOKENS, &mut deps.storage);
    for token in msg.paired_tokens {
        // Value is irrelevant, just marking that token as supported
        supported_tokens.set(token.0.as_bytes(), &[1]);
    }

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
        } => receive_swap_data(deps, asset_in, asset_out, account),
    }
}

fn receive_swap_data<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
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

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {}
}
