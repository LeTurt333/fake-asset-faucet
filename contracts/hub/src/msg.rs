use cosmwasm_schema::{cw_serde, QueryResponses};
//use cosmwasm_std::{StdError, StdResult, Uint128, Empty};
//use cw20::{Cw20Coin, Logo, MinterResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
//pub use cw20::Cw20ExecuteMsg as ExecuteMsg;
//pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cw_serde]
#[cfg_attr(test, derive(Default))]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {

    UpdateAdmin{new_admin: String},

    InitFaucetNeonPeepz{code_id: u64},

    InitFaucetShittyKittyz{code_id: u64},

    InitFaucetCw20One{code_id: u64},

    InitFaucetCw20Two{code_id: u64},

    InitFaucetCw20Tre{code_id: u64},

    HitFaucetNft{},

    HitFaucetCw20s{},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetStateResponse)]
    GetState{},
}

#[cw_serde]
pub struct GetStateResponse {
    pub admin: String,
    pub neon_peepz_address: String,
    pub shitty_kittyz_address: String,
    pub cw20_one_faucet_address: String,
    pub cw20_two_faucet_address: String,
    pub cw20_tre_faucet_address: String,
    pub neon_peepz_count: String,
    pub shitty_kittyz_count: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MigrateMsg {}