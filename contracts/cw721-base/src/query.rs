use serde::de::DeserializeOwned;
use serde::Serialize;
use cosmwasm_schema::cw_serde;

use cosmwasm_std::{to_binary, Addr, Binary, BlockInfo, Deps, Env, Order, StdError, StdResult};

use cw721::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, ContractInfoResponse, CustomMsg,
    Cw721Query, 
    Expiration, NftInfoResponse, NumTokensResponse, OperatorsResponse, OwnerOfResponse,
    TokensResponse,
};
use cw_storage_plus::Bound;
use cw_utils::maybe_addr;

use crate::msg::{MinterResponse, QueryMsg};
use crate::state::{Approval, Cw721Contract, TokenInfo};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;
const MAX_ALLTOKENINFO_LIMIT: u32 = 30;


#[cw_serde]
pub struct AllTokensInfoResponse<T> {
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    // ~~~~~~~~ pub token_uri: Option<String>,
    /// You can add any custom metadata here when you extend cw721-base
    // ~~~~~~~~ pub extension: T,
    // ~~~~~~~~ pub token_id: String,
    // vec of tokens owned by address
    // String = token_id
    // T = Any custom metadata
    // Option<String> = token_uri
    pub all_tokens_info: Vec<(String, T, Option<String>)>,
}

impl<'a, T, C, E, Q> Cw721Query<T> for Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
        self.contract_info.load(deps.storage)
    }

    fn num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse> {
        let count = self.token_count(deps.storage)?;
        Ok(NumTokensResponse { count })
    }

    fn nft_info(&self, deps: Deps, token_id: String) -> StdResult<NftInfoResponse<T>> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(NftInfoResponse {
            token_uri: info.token_uri,
            extension: info.extension,
        })
    }

    /*fn all_tokens_info(
        &self, 
        deps: Deps, 
        owner_addr: String, 
        start_after: Option<String>, 
        limit: Option<u32>
    ) -> StdResult<AllTokensInfoResponse<T>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_ALLTOKENINFO_LIMIT) as usize;
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        let owner_addr = deps.api.addr_validate(&owner_addr)?;

        let owners_tokens: Vec<String> = self
            .tokens
            .idx
            .owner
            .prefix(owner_addr)
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|token_id| {
                let info = self.tokens.load(deps.storage, &token_id)?;
                (token_id, info.extension, info.token_uri)
                }
            )
            .collect::<StdResult<Vec<_>>>()?;

        let owners_tokens_info: Vec<(String, T, Option<String>)> = owners_tokens
            .iter()
            .map(|token_id| {
                    let info = self.tokens.load(deps.storage, &token_id)?;
                    (token_id, info.extension, info.token_uri)
                }
            )
            .collect::<StdResult<Vec<_>>>()?;

        Ok(AllTokensInfoResponse {
            all_tokens_info: owners_tokens_info,
        })

    }*/

    fn owner_of(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(OwnerOfResponse {
            owner: info.owner.to_string(),
            approvals: humanize_approvals(&env.block, &info, include_expired),
        })
    }

    /// operators returns all operators owner given access to
    fn operators(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start_addr = maybe_addr(deps.api, start_after)?;
        let start = start_addr.as_ref().map(Bound::exclusive);

        let owner_addr = deps.api.addr_validate(&owner)?;
        let res: StdResult<Vec<_>> = self
            .operators
            .prefix(&owner_addr)
            .range(deps.storage, start, None, Order::Ascending)
            .filter(|r| {
                include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block)
            })
            .take(limit)
            .map(parse_approval)
            .collect();
        Ok(OperatorsResponse { operators: res? })
    }

    fn approval(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        spender: String,
        include_expired: bool,
    ) -> StdResult<ApprovalResponse> {
        let token = self.tokens.load(deps.storage, &token_id)?;

        // token owner has absolute approval
        if token.owner == spender {
            let approval = cw721::Approval {
                spender: token.owner.to_string(),
                expires: Expiration::Never {},
            };
            return Ok(ApprovalResponse { approval });
        }

        let filtered: Vec<_> = token
            .approvals
            .into_iter()
            .filter(|t| t.spender == spender)
            .filter(|t| include_expired || !t.is_expired(&env.block))
            .map(|a| cw721::Approval {
                spender: a.spender.into_string(),
                expires: a.expires,
            })
            .collect();

        if filtered.is_empty() {
            return Err(StdError::not_found("Approval not found"));
        }
        // we expect only one item
        let approval = filtered[0].clone();

        Ok(ApprovalResponse { approval })
    }

    /// approvals returns all approvals owner given access to
    fn approvals(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<ApprovalsResponse> {
        let token = self.tokens.load(deps.storage, &token_id)?;
        let approvals: Vec<_> = token
            .approvals
            .into_iter()
            .filter(|t| include_expired || !t.is_expired(&env.block))
            .map(|a| cw721::Approval {
                spender: a.spender.into_string(),
                expires: a.expires,
            })
            .collect();

        Ok(ApprovalsResponse { approvals })
    }

    fn tokens(
        &self,
        deps: Deps,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        // start is token_id, which is being saved as the primary key in the indexed map

        // so right now my primary key is (Addr, listing_id),,
        // which is what .keys is a range over

        // .prefix_range is setting a range between prefixes, which is 

        let owner_addr = deps.api.addr_validate(&owner)?;
        let tokens: Vec<String> = self
            .tokens
            .idx
            .owner // the multi-index
            .prefix(owner_addr) // the prefix of the multiindex, owner: MultiIndex<'a, --->Addr<---, TokenInfo<T>, String>,
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect::<StdResult<Vec<_>>>()?;

        Ok(TokensResponse { tokens })
    }

    fn all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        let tokens: StdResult<Vec<String>> = self
            .tokens
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| item.map(|(k, _)| k))
            .collect();

        Ok(TokensResponse { tokens: tokens? })
    }

    fn all_nft_info(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<T>> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(AllNftInfoResponse {
            access: OwnerOfResponse {
                owner: info.owner.to_string(),
                approvals: humanize_approvals(&env.block, &info, include_expired),
            },
            info: NftInfoResponse {
                token_uri: info.token_uri,
                extension: info.extension,
            },
        })
    }
}

/// all_tokens_info gets all Tokens + each tokens info owned by an address
pub trait AllTokensInfoExt<T> {
    fn all_tokens_info(
        &self,
        deps: Deps,
        owner_addr: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<AllTokensInfoResponse<T>>;
}

impl<'a, T, C, E, Q> AllTokensInfoExt<T> for Cw721Contract<'a, T, C, E, Q> 
where
    Cw721Contract<'a, T, C, E, Q>: Cw721Query<T>,
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,

{
    fn all_tokens_info(
        &self,
        deps: Deps,
        owner_addr: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<AllTokensInfoResponse<T>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_ALLTOKENINFO_LIMIT) as usize;
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        let owner_addr = deps.api.addr_validate(&owner_addr)?;

        // start after is converting the string passed to bytes, pinning those bytes, then iterating over every 
        // primary key after that pinned byte

        // so if owner_addr owns [token_id: 11, token_id: 23, token_id: 27, token_id: 36]
        // and I pass in "2" as start_after
        // return is [token_id: 23, token_id 27, token_id: 36]
        
        // this is evident as a query with "0" as start_after returns the same response as 
        // a query with " " as start_after
        // & " " = 00 byte, "0" = 30 byte

        let owners_tokens: Vec<String> = self
            .tokens
            .idx
            .owner
            .prefix(owner_addr)
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect::<StdResult<Vec<_>>>()?;

        let owners_tokens_info: Vec<(String, T, Option<String>)> = owners_tokens
            .iter()
            .map(|token_id| -> StdResult<(String, T, Option<String>)> {
                    let info = self.tokens.load(deps.storage, &token_id);

                    match info {
                        Ok(x) => {return Ok((token_id.clone(), x.extension, x.token_uri))},
                        Err(e) => {return Err(e)},
                    }
                }
            )
            .collect::<StdResult<Vec<_>>>()?;

        Ok(AllTokensInfoResponse {
            all_tokens_info: owners_tokens_info,
        })
    }
}

impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn minter(&self, deps: Deps) -> StdResult<MinterResponse> {
        let minter_addr = self.minter.load(deps.storage)?;
        Ok(MinterResponse {
            minter: minter_addr.to_string(),
        })
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg<Q>) -> StdResult<Binary> {
        match msg {
            QueryMsg::Minter {} => to_binary(&self.minter(deps)?),
            QueryMsg::ContractInfo {} => to_binary(&self.contract_info(deps)?),
            QueryMsg::NftInfo { token_id } => to_binary(&self.nft_info(deps, token_id)?),
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => {
                to_binary(&self.owner_of(deps, env, token_id, include_expired.unwrap_or(false))?)
            }
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => to_binary(&self.all_nft_info(
                deps,
                env,
                token_id,
                include_expired.unwrap_or(false),
            )?),
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => to_binary(&self.operators(
                deps,
                env,
                owner,
                include_expired.unwrap_or(false),
                start_after,
                limit,
            )?),
            QueryMsg::NumTokens {} => to_binary(&self.num_tokens(deps)?),
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => to_binary(&self.tokens(deps, owner, start_after, limit)?),
            QueryMsg::AllTokens { start_after, limit } => {
                to_binary(&self.all_tokens(deps, start_after, limit)?)
            }
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => to_binary(&self.approval(
                deps,
                env,
                token_id,
                spender,
                include_expired.unwrap_or(false),
            )?),
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => {
                to_binary(&self.approvals(deps, env, token_id, include_expired.unwrap_or(false))?)
            }
            QueryMsg::Extension { msg: _ } => Ok(Binary::default()),
            QueryMsg::AllTokensInfo { owner_addr, start_after, limit } => {
                to_binary(&self.all_tokens_info(deps, owner_addr, start_after, limit)?)
            }
        }
    }
}

fn parse_approval(item: StdResult<(Addr, Expiration)>) -> StdResult<cw721::Approval> {
    item.map(|(spender, expires)| cw721::Approval {
        spender: spender.to_string(),
        expires,
    })
}

fn humanize_approvals<T>(
    block: &BlockInfo,
    info: &TokenInfo<T>,
    include_expired: bool,
) -> Vec<cw721::Approval> {
    info.approvals
        .iter()
        .filter(|apr| include_expired || !apr.is_expired(block))
        .map(humanize_approval)
        .collect()
}

fn humanize_approval(approval: &Approval) -> cw721::Approval {
    cw721::Approval {
        spender: approval.spender.to_string(),
        expires: approval.expires,
    }
}
