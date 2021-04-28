use crate::contract::is_scrt;
use crate::querier::query_pair;
use cosmwasm_std::{Api, Extern, HumanAddr, Querier, StdError, StdResult, Storage};
use cosmwasm_storage::PrefixedStorage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const KEY_SSCRT: &[u8] = b"sscrt";
pub const KEY_CSHBK: &[u8] = b"cshbk";
pub const KEY_ADMIN: &[u8] = b"admin";
pub const PREFIX_PAIRED_TOKENS: &[u8] = b"pairedtokens";

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, JsonSchema)]
pub struct Pair {
    pub asset_0: HumanAddr,
    pub asset_1: HumanAddr,
}

pub fn set_pairs_to_storage<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    pairs: Vec<HumanAddr>,
    pair_hash: String,
) -> StdResult<()> {
    for pair in pairs {
        let pair_info = query_pair(&deps.querier, pair.clone(), pair_hash.clone())?;
        if !is_scrt(&deps, pair_info.asset_infos[0].clone())?
            && !is_scrt(&deps, pair_info.asset_infos[1].clone())?
        {
            return Err(StdError::generic_err(
                "one of the sides of the pair has to be SCRT",
            ));
        }

        // Value is irrelevant, just marking that pair as supported
        let mut supported_tokens = PrefixedStorage::new(PREFIX_PAIRED_TOKENS, &mut deps.storage);
        supported_tokens.set(pair.0.as_bytes(), &[1]);
    }

    Ok(())
}

pub fn remove_pairs_from_storage<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    pairs: Vec<HumanAddr>,
) -> StdResult<()> {
    let mut supported_tokens = PrefixedStorage::new(PREFIX_PAIRED_TOKENS, &mut deps.storage);
    for pair in pairs {
        supported_tokens.remove(pair.0.as_bytes());
    }

    Ok(())
}
