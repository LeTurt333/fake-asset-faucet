#[cfg(not(feature = "library"))]
// The Essentials
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, from_binary, Deps, DepsMut, Env,
    MessageInfo, Response, Addr, Uint128,
    CosmosMsg, WasmMsg, Empty,
    SubMsg, ReplyOn, SubMsgResult,  Reply, 
    coin, has_coins, Binary, StdResult, Coin,
}; // Attribute, QueryRequest, WasmQuery

use cw_utils::{
    ParseReplyError, parse_reply_instantiate_data, parse_reply_execute_data,
}; // MsgExecuteContractResponse

use cw2::set_contract_version;

use cw721_neon_peepz::{
    InstantiateMsg as Cw721NeonPeepzInstantiateMsg,
};

use cw721_shitty_kittyz::{
    InstantiateMsg as Cw721ShittyKittyzInstantiateMsg,
};

use cw20_base::msg::{
    InstantiateMsg as Cw20InitMsg,
    ExecuteMsg as Cw20ExecuteMsg,
};

//use cw20::Cw20QueryMsg::Minter;
use cw20::MinterResponse;

// Nois
use nois::{ 
    NoisCallback, ProxyExecuteMsg, MAX_JOB_ID_LEN, sub_randomness_with_key, 
};
//use serde::{Serialize, Deserialize};

use serde::Deserialize;
use serde::Serialize;
// use serde_json_wasm::*;
// use serde_cw_value::*;
// use serde::de::DeserializeOwned;

// The Commons
use crate::msg::*;
use crate::state::*;
use crate::error::*;
use crate::utils::*;
use std::str;

// Contract name used for migration
const CONTRACT_NAME: &str = "crates.io:fake-faucet-hub";
// Contract version thats used for migration
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

////////////////////////////////////////////////////////////////////////////////////////

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Instantiate
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = msg.admin.unwrap_or_else(|| info.sender.to_string());

    let validated = deps.api.addr_validate(&admin)?;

    //juno1tquqqdvlv3fwu5u6evpt7e4ss47zczug8tq4czjucgx8dulkhjxsegfuds
    
    //juno1v82su97skv6ucfqvuvswe0t5fph7pfsrtraxf0x33d8ylj5qnrysdvkc95
    let nois_proxy_addr = deps
        .api
        .addr_validate(&msg.nois_proxy)
        .map_err(|_| ContractError::InvalidProxyAddress)?;
        
    CONFIGURATION.save(deps.storage, &Configuration{
        admin: validated,
        neon_peepz_addy: None,
        shitty_kittyz_addy: None,
        cw20_one_faucet_addy: None,
        cw20_two_faucet_addy: None,
        cw20_tre_faucet_addy: None,
        nois_beacon: nois_proxy_addr,
    })?;

    NPCOUNT.save(deps.storage, &1)?;
    SKCOUNT.save(deps.storage, &1)?;

    JOBINCREMENT.save(deps.storage, &1)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", admin)
    )
}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Execute
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        
        ExecuteMsg::UpdateAdmin {new_admin} => execute_update_admin(deps, &info.sender, new_admin),
        ExecuteMsg::UpdateNois { new_addr } => execute_update_nois(deps, &info.sender, new_addr),

        ExecuteMsg::InitFaucetNeonPeepz{code_id} => execute_add_faucet_neon_peepz(deps, env, &info.sender, code_id),
        ExecuteMsg::InitFaucetShittyKittyz{code_id} => execute_add_faucet_shitty_kittyz(deps, env, &info.sender, code_id),
        ExecuteMsg::InitFaucetCw20One{code_id} => execute_add_faucet_cw20(deps, env, &info.sender, code_id),
        ExecuteMsg::InitFaucetCw20Two{code_id} => execute_add_faucet_cw20_two(deps, env, &info.sender, code_id),
        ExecuteMsg::InitFaucetCw20Tre{code_id} => execute_add_faucet_cw20_tre(deps, env, &info.sender, code_id),

        
        ExecuteMsg::HitFaucetNft{} => execute_hit_faucet_nft(deps, info, env),
        ExecuteMsg::HitFaucetCw20s{} => execute_hit_faucet_cw20s(deps.as_ref(), info, env),

        // Nois Callback
        ExecuteMsg::NoisReceive { callback } => execute_nois_callback(deps, env, info, callback),
    }
}

////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//// Only callable by Admin
////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub fn execute_update_admin(
    deps:DepsMut, 
    sender: &Addr, 
    new_admin: String
) -> Result<Response, ContractError> {

    let config = CONFIGURATION.load(deps.storage)?;

    if &config.admin != sender {
        return Err(ContractError::Unauthorized{});
    }

    let new_adminz = deps.api.addr_validate(&new_admin)?;

    CONFIGURATION.update(
        deps.storage,
        |old| -> Result<Configuration, ContractError> {
            Ok(Configuration{
                admin: new_adminz,
                ..old
            })
        }
    )?;


    Ok(Response::new().add_attribute("new_admin", new_admin))
}

// Instantiates the neon_peepz contract, stores nft_faucet_addy on reply
pub fn execute_add_faucet_neon_peepz(
    deps: DepsMut,
    env: Env,
    sender: &Addr,
    code_id: u64,
) -> Result<Response, ContractError> {

    let config = CONFIGURATION.load(deps.storage)?;

    if &config.admin != sender {
        return Err(ContractError::Unauthorized{});
    }

    let cw721initmsg = Cw721NeonPeepzInstantiateMsg {
        name: "NeoNPeePz".to_string(),
        symbol: "NEONPEEPZ".to_string(),
        minter: env.contract.address.to_string(),
    };

    let bin_cw721_init_msg = to_binary(&cw721initmsg)?;

    let cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Instantiate {
        admin: None,
        code_id: code_id,
        msg: bin_cw721_init_msg,
        funds: vec![],
        label: "NeoNPeePz Contract".to_string(),
    });

    let sub_msg = SubMsg {
        id: 1,
        msg: cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    Ok(Response::new()
        .add_attribute("Instantiate NeonPeepz faucet from Hub", "innit")
        .add_submessage(sub_msg)
    )
}

// Instantiates the shitty_kittyz contract, stores nft_faucet_addy on reply
pub fn execute_add_faucet_shitty_kittyz(
    deps: DepsMut,
    env: Env,
    sender: &Addr,
    code_id: u64,
) -> Result<Response, ContractError> {

    let config = CONFIGURATION.load(deps.storage)?;

    if &config.admin != sender {
        return Err(ContractError::Unauthorized{});
    }

    let cw721initmsg = Cw721ShittyKittyzInstantiateMsg {
        name: "ShittyKittyz".to_string(),
        symbol: "SHITKIT".to_string(),
        minter: env.contract.address.to_string(),
    };

    let bin_cw721_init_msg = to_binary(&cw721initmsg)?;

    let cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Instantiate {
        admin: None,
        code_id: code_id,
        msg: bin_cw721_init_msg,
        funds: vec![],
        label: "Shitty Kittyz Contract".to_string(),
    });

    let sub_msg = SubMsg {
        id: 2,
        msg: cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    Ok(Response::new()
        .add_attribute("Instantiate ShittyKittyz faucet from Hub", "innit")
        .add_submessage(sub_msg)
    )
}

// Instantiates the cw20 contract with $JVONE, stores cw20_one_faucet_addy on reply
pub fn execute_add_faucet_cw20(
    deps: DepsMut,
    env: Env,
    sender: &Addr,
    code_id: u64,
) -> Result<Response, ContractError> {

    let config = CONFIGURATION.load(deps.storage)?;

    if &config.admin != sender {
        return Err(ContractError::Unauthorized{});
    }

    let minter = MinterResponse {
        minter: env.contract.address.to_string(),
        cap: None,
    };

    let cw20initmsg = Cw20InitMsg {
        name: "JV-One".to_string(),
        symbol: "JVONE".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(minter),
        marketing: None,
    };

    let bin_cw20_init_msg = to_binary(&cw20initmsg)?;

    let cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Instantiate {
        admin: Some(config.admin.to_string()),
        code_id: code_id,
        msg: bin_cw20_init_msg,
        funds: vec![],
        label: "JV-One cw20 contract".to_string(),
    });

    let sub_msg = SubMsg {
        id: 11,
        msg: cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    Ok(Response::new()
        .add_attribute("Instantiate JV-One from Hub", "innit")
        .add_submessage(sub_msg)
    )
}

// Instantiates the cw20 contract with $JVTWO, stores cw20_two_faucet_addy on reply
pub fn execute_add_faucet_cw20_two(
    deps: DepsMut,
    env: Env,
    sender: &Addr,
    code_id: u64,
) -> Result<Response, ContractError> {

    let config = CONFIGURATION.load(deps.storage)?;

    if &config.admin != sender {
        return Err(ContractError::Unauthorized{});
    }

    let minter = MinterResponse {
        minter: env.contract.address.to_string(),
        cap: None,
    };

    let cw20initmsg = Cw20InitMsg {
        name: "JV-Two".to_string(),
        symbol: "JVTWO".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(minter),
        marketing: None,
    };

    let bin_cw20_init_msg = to_binary(&cw20initmsg)?;

    let cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Instantiate {
        admin: Some(config.admin.to_string()),
        code_id: code_id,
        msg: bin_cw20_init_msg,
        funds: vec![],
        label: "JV-Two cw20 contract".to_string(),
    });

    let sub_msg = SubMsg {
        id: 12,
        msg: cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    Ok(Response::new()
        .add_attribute("Instantiate JV-Two from Hub", "innit")
        .add_submessage(sub_msg)
    )
}

// Instantiates the cw20 contract with $JVTRE, stores cw20_two_faucet_addy on reply
pub fn execute_add_faucet_cw20_tre(
    deps: DepsMut,
    env: Env,
    sender: &Addr,
    code_id: u64,
) -> Result<Response, ContractError> {

    let config = CONFIGURATION.load(deps.storage)?;

    if &config.admin != sender {
        return Err(ContractError::Unauthorized{});
    }

    let minter = MinterResponse {
        minter: env.contract.address.to_string(),
        cap: None,
    };

    let cw20initmsg = Cw20InitMsg {
        name: "JV-Tre".to_string(),
        symbol: "JVTRE".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(minter),
        marketing: None,
    };

    let bin_cw20_init_msg = to_binary(&cw20initmsg)?;

    let cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Instantiate {
        admin: Some(config.admin.to_string()),
        code_id: code_id,
        msg: bin_cw20_init_msg,
        funds: vec![],
        label: "JV-Tre cw20 contract".to_string(),
    });

    let sub_msg = SubMsg {
        id: 13,
        msg: cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    Ok(Response::new()
        .add_attribute("Instantiate JV-Tre from Hub", "innit")
        .add_submessage(sub_msg)
    )
}

pub fn execute_update_nois(
    deps: DepsMut,
    sender: &Addr,
    new_addr: String,
) -> Result<Response, ContractError> {

    let config: Configuration = CONFIGURATION.load(deps.storage)?;

    if &config.admin != sender {
        return Err(ContractError::Unauthorized{});
    }

    let validated_nois: Addr = deps.api.addr_validate(&new_addr)?;

    CONFIGURATION.update(
        deps.storage,
        |old| -> Result<Configuration, ContractError> {
            return Ok(Configuration {nois_beacon: validated_nois,
                ..old
            }
        );}
    )?;


    Ok(Response::new().add_attribute("update_nois", format!("new_addr: {}", new_addr)))


}

////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//// Callable by anyone
////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub fn execute_hit_faucet_cw20s(
    deps: Deps,
    info: MessageInfo,
    _env: Env,
) -> Result<Response, ContractError> {

    // pull faucet address' from config
    let config = CONFIGURATION.load(deps.storage)?;
    let jv_one_faucet = config.cw20_one_faucet_addy.ok_or(ContractError::NoFaucetAddy{})?;
    let jv_two_faucet = config.cw20_two_faucet_addy.ok_or(ContractError::NoFaucetAddy{})?;
    let jv_tre_faucet = config.cw20_tre_faucet_addy.ok_or(ContractError::NoFaucetAddy{})?;

    // check that message contains enough ujunox or is admin
    let cost_juno = coin(5000000, "ujunox");
    if !has_coins(&info.funds, &cost_juno) && info.sender != config.admin {
        return Err(ContractError::Unauthorized{});
    }

    // Create mint msgs to sent to cw20 faucet addresses
    let jvone_mintmsg = Cw20ExecuteMsg::Mint {
        amount: Uint128::from(69000000u128),
        recipient: info.sender.to_string(),
    };
    let jvtwo_mintmsg = Cw20ExecuteMsg::Mint {
        amount: Uint128::from(69000000u128),
        recipient: info.sender.to_string(),
    };
    let jvtre_mintmsg = Cw20ExecuteMsg::Mint {
        amount: Uint128::from(69000000u128),
        recipient: info.sender.to_string(),
    };

    let bin_jvone = to_binary(&jvone_mintmsg)?;
    let bin_jvtwo = to_binary(&jvtwo_mintmsg)?;
    let bin_jvtre = to_binary(&jvtre_mintmsg)?;

    let jvone_cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Execute {
        contract_addr: jv_one_faucet.to_string(),
        funds: vec![],
        msg: bin_jvone,
    });
    let jvtwo_cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Execute {
        contract_addr: jv_two_faucet.to_string(),
        funds: vec![],
        msg: bin_jvtwo,
    });
    let jvtre_cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Execute {
        contract_addr: jv_tre_faucet.to_string(),
        funds: vec![],
        msg: bin_jvtre,
    });

    Ok(Response::new()
        .add_attribute("Hit jv20 faucets", "JVONE JVTWO JVTRE")
        .add_message(jvone_cosmos_msg)
        .add_message(jvtwo_cosmos_msg)
        .add_message(jvtre_cosmos_msg)
    )
}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Nois callback
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ad = user adderss,
// id = job number
// user_address can't be used as a field here as it would exceed
// the max usize config in Nois Proxy contract
#[derive(Serialize, Deserialize)]
pub struct JobId {
    ad: String,
    id: u32
}

pub fn execute_hit_faucet_nft(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    //job_id: String
) -> Result<Response, ContractError> {

    // Pull config
    let config: Configuration = CONFIGURATION.load(deps.storage)?;

    // check that message contains enough ujunox or is admin
    let cost_juno = coin(5000000, "ujunox");
    if !has_coins(&info.funds, &cost_juno) && info.sender != config.admin {
        return Err(ContractError::Unauthorized{});
    }

    // Get the Nois Proxy contract fee price
    // let resp: ? = deps
    //     .querier
    //     .query_wasm_smart(
    //         config.nois_beacon, 
    //         &"{prices: {denom: ujunox}}".to_string()
    //     )?;

    // As this call is isolated (does not involve submsgs),
    // State is only reverted if this call or the Nois Proxy call fails -
    // If there is an error in handling the Callback from the Nois Proxy, it will not revert
    // this fee payment nor the payment the user made (5 junox) 
    let nois_fee: Coin = coin(150, "ujunox");

    // Pull count for job_id 
    let jobincrement = JOBINCREMENT.load(deps.storage)?;

    // Create serializable struct that contains sender address & job_id,
    // since we will need both in the Nois callback execution
    let job_id: JobId = JobId {
        ad: info.sender.to_string(),
        id: jobincrement
    };

    let serialized_job_id = serde_json_wasm::to_string(&job_id).map_err(|_| ContractError::SerializeError)?;

    // Check that Nois job_id doesn't exceed max length
    // manually prechecked but doesn't hurt
    if serialized_job_id.len() > MAX_JOB_ID_LEN {
        return Err(ContractError::JobIdTooLong);
    }

    let execute_nois_msg = WasmMsg::Execute {
        contract_addr: config.nois_beacon.into(),
        //GetNextRandomness requests the randomness from the proxy
        //The job id is needed to know what randomness we are referring to upon reception in the callback
        msg: to_binary(&ProxyExecuteMsg::GetNextRandomness { job_id: serialized_job_id})?,
        // We pay here the contract with the native chain coin.
        funds: vec![nois_fee]
    };

    // JOBINCREMENT must be updated here since execute_nois_msg is not a submsg
    // & more importantly each key in sub_randomness needs to be different
    JOBINCREMENT.update(
        deps.storage,
        |old| -> Result<u32, ContractError> {
            if old >= u32::MAX - 1 {
                Ok(1)
            } else {
                Ok(old + 1)
            }
        }
    ).map_err(|_| ContractError::NoJobIncrement)?;

    Ok(Response::new().add_message(execute_nois_msg))

}

pub fn execute_nois_callback(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    callback: NoisCallback
) -> Result<Response, ContractError> {

    // Pull config
    let config: Configuration = CONFIGURATION.load(deps.storage)?;

    // Verify nois contract is caller
    if info.sender != config.nois_beacon {
        return Err(ContractError::Unauthorized {  });
    }

    // Deserialize job_id into JobId struct
    let de_job_id: JobId = serde_json_wasm::from_str(&callback.job_id).map_err(|_| ContractError::DeserializeError)?;

    // Validate user wallet
    let user_wallet = deps.api.addr_validate(&de_job_id.ad)?;

    // Get base randomness from NoisCallback
    let base_rand: [u8; 32] = callback
        .randomness
        .to_array()
        .map_err(|_| ContractError::InvalidRandomness)?;

    // Create subrandomness provider
    let mut provider = sub_randomness_with_key(base_rand, de_job_id.id.to_string());
    // need to deref out of Box here?
    let randomness = provider.provide(); 
    
    // pull token counts from state to use as the token_id
    let np_count = NPCOUNT.load(deps.storage)?;
    let sk_count = SKCOUNT.load(deps.storage)?;

    // get faucet addresses
    let np_faucet_addr = config.neon_peepz_addy.ok_or(ContractError::NoFaucetAddy{})?;
    let sk_faucet_addr = config.shitty_kittyz_addy.ok_or(ContractError::NoFaucetAddy{})?;

    // Make mint submsgs
    let mint_submsgs: Vec<SubMsg> = make_mint_submsgs(
        np_faucet_addr,
        np_count,
        sk_faucet_addr,
        sk_count,
        user_wallet,
        randomness
    )?;

    // Increment np_count & sk_count on submsg replies
    Ok(Response::new()
        .add_attribute("Mint two NFTs from hub", "minnit")
        .add_submessages(mint_submsgs)
    )
}


//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Submessage Reply
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[entry_point]
pub fn reply(
    deps: DepsMut, 
    _env: Env, 
    msg: Reply
) -> Result<Response, ContractError> {

    match msg.id {

        // Reply from NeonPeepz Contract Init
        1 => {
            let res = parse_reply_instantiate_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "Init_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "Init_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "Init_BrokenUtf8".to_string(), v: "n/e".to_string()}
                    },
                }            
            })?;
        
            if let SubMsgResult::Err(x) = msg.result {
                return Err(ContractError::SubMsgUncaughtErr{st: x});
            }

            let child_contract = deps.api.addr_validate(&res.contract_address)?;
                
            CONFIGURATION.update(
                deps.storage,
                |old| -> Result<Configuration, ContractError> {
                Ok(Configuration {
                    neon_peepz_addy: Some(child_contract.clone()),
                    ..old
                })}
            )?;

            return Ok(Response::new()
                .add_attribute("Init NeonPeepz Reply Success", format!("Faucet: {}", child_contract.to_string()))
            );
        },

        // Reply from ShittyKittyz Contract Init
        2 => {
            let res = parse_reply_instantiate_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "Init_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "Init_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "Init_BrokenUtf8".to_string(), v: "n/e".to_string()}
                    },
                }            
            })?;
        
            if let SubMsgResult::Err(x) = msg.result {
                return Err(ContractError::SubMsgUncaughtErr{st: x});
            }

            let child_contract = deps.api.addr_validate(&res.contract_address)?;
                
            CONFIGURATION.update(
                deps.storage,
                |old| -> Result<Configuration, ContractError> {
                Ok(Configuration {
                    shitty_kittyz_addy: Some(child_contract.clone()),
                    ..old
                })}
            )?;

            return Ok(Response::new()
                .add_attribute("Init ShittyKittyz Reply Success", format!("Faucet: {}", child_contract.to_string()))
            );
        },

        // Reply from NeonPeep MintMsg
        3 => {
            let res = parse_reply_execute_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "NP_Mint_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "NP_Mint_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "NP_Mint_BrokenUtf8".to_string(), v: "n/e".to_string()}
                    },
                }            
            })?;

            if let SubMsgResult::Err(x) = msg.result {
                return Err(ContractError::SubMsgUncaughtErr{st: x});
            }

            // Only change to state is updating the count for next NFTs token_id
            // Update this directly from storage <rather than from token_id>,
            // since its unknown whether or not another SubMsgReply has been processed first
            NPCOUNT.update(
                deps.storage, 
                |old| -> Result<u32, ContractError> {
                    Ok(old + 1)
                }
            )?;

            if let Some(_bin) = res.data.clone() {
                let x = res.data.unwrap();
                let token_id = from_binary::<u32>(&x)?;
                return Ok(Response::new()
                    .add_attribute("Mint NeonPeep from hub", "Success")
                    .add_attribute("NFT Token ID: ", token_id.to_string())
                );
            } else {
                return Ok(Response::new()
                    .add_attribute("Mint NeonPeep from hub", "Success")
                    .add_attribute("No Token ID Found: ", "Query the Base cw721 contract directly")
                );
            }
        },

        // Reply from SK MintMsg
        4 => {
            let res = parse_reply_execute_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "SK_Mint_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "SK_Mint_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "SK_Mint_BrokenUtf8".to_string(), v: "n/e".to_string()}
                    },
                }            
            })?;

            if let SubMsgResult::Err(x) = msg.result {
                return Err(ContractError::SubMsgUncaughtErr{st: x});
            }

            // Only change to state is updating the count for next NFTs token_id
            // Update this directly from storage <rather than from token_id>,
            // since its unknown whether or not another SubMsgReply has been processed first
            SKCOUNT.update(
                deps.storage, 
                |old| -> Result<u32, ContractError> {
                    Ok(old + 1)
                }
            )?;

            // NOTE - Events no longer returned because undeterminst from Authz
            // Now grab attributes from SubMsg response to return them
            //let events = msg.result.unwrap().events;
            //let vec_attributes = events
            //    .iter()
            //    .map(|event| event.attributes.clone())
            //    .flatten()
            //    .collect::<Vec<Attribute>>();


            if let Some(_bin) = res.data.clone() {
                let x = res.data.unwrap();
                let token_id = from_binary::<u32>(&x)?;
                return Ok(Response::new()
                    .add_attribute("Mint ShittyKitty from hub", "Success")
                    .add_attribute("NFT Token ID: ", token_id.to_string())
                );
            } else {
                return Ok(Response::new()
                    .add_attribute("Mint ShittyKitty from hub", "Success")
                    .add_attribute("No Token ID Found: ", "Query the Base cw721 contract directly")
                );
            }

        },

        // Reply from JV-One Contract Init
        11 => {
            let res = parse_reply_instantiate_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "JVONE_Init_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "JVONE_Init_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "JVONE_Init_BrokenUtf8".to_string(), v: "n/e".to_string()}
                    },
                }            
            })?;
        
            if let SubMsgResult::Err(x) = msg.result {
                return Err(ContractError::SubMsgUncaughtErr{st: x});
            }

            let child_contract = deps.api.addr_validate(&res.contract_address)?;
                
            CONFIGURATION.update(
                deps.storage,
                |old| -> Result<Configuration, ContractError> {
                Ok(Configuration {
                    cw20_one_faucet_addy: Some(child_contract.clone()),
                    ..old
                })}
            )?;

            return Ok(Response::new()
                .add_attribute("Init JV-One Reply Success", format!("Faucet: {}", child_contract.to_string()))
            );

        },

        // Reply from JV-Two Contract Init
        12 => {
            let res = parse_reply_instantiate_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "JVTWO_Init_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "JVTWO_Init_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "JVTWO_Init_BrokenUtf8".to_string(), v: "n/e".to_string()}
                    },
                }            
            })?;
        
            if let SubMsgResult::Err(x) = msg.result {
                return Err(ContractError::SubMsgUncaughtErr{st: x});
            }

            let child_contract = deps.api.addr_validate(&res.contract_address)?;
                
            CONFIGURATION.update(
                deps.storage,
                |old| -> Result<Configuration, ContractError> {
                Ok(Configuration {
                    cw20_two_faucet_addy: Some(child_contract.clone()),
                    ..old
                })}
            )?;

            return Ok(Response::new()
                .add_attribute("Init JV-Two Reply Success", format!("Faucet: {}", child_contract.to_string()))
            );

        },

        // Reply from JV-Tre contract Init
        13 => {
            let res = parse_reply_instantiate_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "JVTRE_Init_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "JVTRE_Init_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "JVTRE_Init_BrokenUtf8".to_string(), v: "n/e".to_string()}
                    },
                }            
            })?;
        
            if let SubMsgResult::Err(x) = msg.result {
                return Err(ContractError::SubMsgUncaughtErr{st: x});
            }

            let child_contract = deps.api.addr_validate(&res.contract_address)?;
                
            CONFIGURATION.update(
                deps.storage,
                |old| -> Result<Configuration, ContractError> {
                Ok(Configuration {
                    cw20_tre_faucet_addy: Some(child_contract.clone()),
                    ..old
                })}
            )?;

            return Ok(Response::new()
                .add_attribute("Init JV-Tre Reply Success", format!("Faucet: {}", child_contract.to_string()))
            );

        },

        // Invalid reply msg.id
        _ => {
            return Err(ContractError::InvalidId{x: msg.id});
        },

    }
}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Query
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState{} => to_binary(&get_state(deps)?),
    }
}

pub fn get_state(deps: Deps) -> StdResult<Binary> {

    let config = CONFIGURATION.load(deps.storage)?;

    let admin = config.admin.to_string();

    let neon_peepz_address = config
        .neon_peepz_addy
        .map_or_else(|| "None".to_string(), |addy| addy.to_string());
    
    let shitty_kittyz_address = config
        .shitty_kittyz_addy
        .map_or_else(|| "None".to_string(), |addy| addy.to_string());

    let cw20_one_faucet_address = config
        .cw20_one_faucet_addy
        .map_or_else(|| "None".to_string(), |addy| addy.to_string());

    let cw20_two_faucet_address = config
        .cw20_two_faucet_addy
        .map_or_else(|| "None".to_string(), |addy| addy.to_string());

    let cw20_tre_faucet_address = config
        .cw20_tre_faucet_addy
        .map_or_else(|| "None".to_string(), |addy| addy.to_string());

    let neon_peepz_count = NPCOUNT.load(deps.storage)?.to_string();
    let shitty_kittyz_count = SKCOUNT.load(deps.storage)?.to_string();
    let job_count = JOBINCREMENT.load(deps.storage)?.to_string();

    to_binary(&GetStateResponse {
        admin,
        neon_peepz_address,
        shitty_kittyz_address,
        cw20_one_faucet_address,
        cw20_two_faucet_address,
        cw20_tre_faucet_address,
        nois_proxy_address: config.nois_beacon.to_string(),
        neon_peepz_count,
        shitty_kittyz_count,
        job_count
    })

}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Tests
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        let a = true;
        assert_eq!(a, true);
    }
}