#[cfg(not(feature = "library"))]
// The Essentials
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, from_binary, Deps, DepsMut, Env, 
    MessageInfo, Response, Addr, Uint128,
    CosmosMsg, WasmMsg, Empty,
    SubMsg, ReplyOn, SubMsgResult,  Reply, 
    coin, has_coins, Binary, StdResult
}; // Attribute

use cw_utils::{
    ParseReplyError, parse_reply_instantiate_data, parse_reply_execute_data,
}; // MsgExecuteContractResponse

use cw2::set_contract_version;

use cw721_neon_peepz::{
    ExecuteMsg as Cw721NeonPeepzExecuteMsg,
    InstantiateMsg as Cw721NeonPeepzInstantiateMsg,
    MintMsg as NeonPeepzMintMsg,
    NeonPeepzExtension,
    RandomNP,
};

use cw721_shitty_kittyz::{
    ExecuteMsg as Cw721ShittyKittyzExecuteMsg,
    InstantiateMsg as Cw721ShittyKittyzInstantiateMsg,
    MintMsg as ShittyKittyzMintMsg,
    ShittyKittyzExtension,
    RandomSK,
};

use cw20_base::msg::{
    InstantiateMsg as Cw20InitMsg,
    ExecuteMsg as Cw20ExecuteMsg,
};

//use cw20::Cw20QueryMsg::Minter;
use cw20::MinterResponse;

// The Commons
use crate::msg::*;
use crate::state::*;
use crate::error::*;
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
        
    CONFIGURATION.save(deps.storage, &Configuration{
        admin: validated,
        neon_peepz_addy: None,
        shitty_kittyz_addy: None,
        cw20_one_faucet_addy: None,
        cw20_two_faucet_addy: None,
        cw20_tre_faucet_addy: None,
    })?;

    NPCOUNT.save(deps.storage, &1)?;
    SKCOUNT.save(deps.storage, &1)?;

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
        // ~~~~~~~~~~~~~~~~ Callable by admin
        ExecuteMsg::UpdateAdmin {new_admin} => execute_update_admin(deps, &info.sender, new_admin),
        ExecuteMsg::InitFaucetNeonPeepz{code_id} => execute_add_faucet_neon_peepz(deps, env, &info.sender, code_id),
        ExecuteMsg::InitFaucetShittyKittyz{code_id} => execute_add_faucet_shitty_kittyz(deps, env, &info.sender, code_id),
        ExecuteMsg::InitFaucetCw20One{code_id} => execute_add_faucet_cw20(deps, env, &info.sender, code_id),
        ExecuteMsg::InitFaucetCw20Two{code_id} => execute_add_faucet_cw20_two(deps, env, &info.sender, code_id),
        ExecuteMsg::InitFaucetCw20Tre{code_id} => execute_add_faucet_cw20_tre(deps, env, &info.sender, code_id),
        // ~~~~~~~~~~~~~~~~ Callable by anyone
        ExecuteMsg::HitFaucetNft{} => execute_hit_faucet_nft(deps.as_ref(), info, env),
        ExecuteMsg::HitFaucetCw20s{} => execute_hit_faucet_cw20s(deps.as_ref(), info, env),
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

// Instantiates the cw20 contract with $CSONE, stores cw20_one_faucet_addy on reply
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
        name: "CS-One".to_string(),
        symbol: "CSONE".to_string(),
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
        label: "CS-One cw20 contract".to_string(),
    });

    let sub_msg = SubMsg {
        id: 11,
        msg: cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    Ok(Response::new()
        .add_attribute("Instantiate CS-One from Hub", "innit")
        .add_submessage(sub_msg)
    )
}

// Instantiates the cw20 contract with $CSTWO, stores cw20_two_faucet_addy on reply
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
        name: "CS-Two".to_string(),
        symbol: "CSTWO".to_string(),
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
        label: "CS-Two cw20 contract".to_string(),
    });

    let sub_msg = SubMsg {
        id: 12,
        msg: cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    Ok(Response::new()
        .add_attribute("Instantiate CS-Two from Hub", "innit")
        .add_submessage(sub_msg)
    )
}

// Instantiates the cw20 contract with $CSTRE, stores cw20_two_faucet_addy on reply
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
        name: "CS-Tre".to_string(),
        symbol: "CSTRE".to_string(),
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
        label: "CS-Tre cw20 contract".to_string(),
    });

    let sub_msg = SubMsg {
        id: 13,
        msg: cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    Ok(Response::new()
        .add_attribute("Instantiate CS-Tre from Hub", "innit")
        .add_submessage(sub_msg)
    )
}


////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//// Callable by anyone
////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub fn execute_hit_faucet_nft(
    deps: Deps,
    info: MessageInfo,
    env: Env,
) -> Result<Response, ContractError> {

    // pull faucet address from config
    let config = CONFIGURATION.load(deps.storage)?;
    let np_faucet_addr = config.neon_peepz_addy.ok_or(ContractError::NoFaucetAddy{})?;
    let sk_faucet_addr = config.shitty_kittyz_addy.ok_or(ContractError::NoFaucetAddy{})?;

    // check that message contains enough ujunox or is admin
    let cost_juno = coin(5000000, "ujunox");
    if !has_coins(&info.funds, &cost_juno) && info.sender != config.admin {
        return Err(ContractError::Unauthorized{});
    }

    // pull token count from state within this contract,
    // rather than on the cw721 contract directly,
    // to use as the token_id
    let np_count = NPCOUNT.load(deps.storage)?;
    let sk_count = SKCOUNT.load(deps.storage)?;

    // create "random" metadata with blockheight
    // Note - this can be gamed, not a real solution like drand or Nois
    let np_rand_metadata = NeonPeepzExtension::rand_metadata_extension_neonpeepz(env.block.height);
    let sk_rand_metadata = ShittyKittyzExtension::rand_metadata_extension_skittyz(env.block.height);

    // create execute message to send to faucet address
    let np_mint_msg: NeonPeepzMintMsg<NeonPeepzExtension> = NeonPeepzMintMsg{
        count: np_count,
        token_id: np_count.to_string(),
        owner: info.sender.to_string(),
        token_uri: None,
        extension: np_rand_metadata,
    };

    let sk_mint_msg: ShittyKittyzMintMsg<ShittyKittyzExtension> = ShittyKittyzMintMsg{
        count: sk_count,
        token_id: sk_count.to_string(),
        owner: info.sender.to_string(),
        token_uri: None,
        extension: sk_rand_metadata,
    };

    let np_exec_mint = Cw721NeonPeepzExecuteMsg::Mint(np_mint_msg);
    let sk_exec_mint = Cw721ShittyKittyzExecuteMsg::Mint(sk_mint_msg);

    let np_bin_exec_mint = to_binary(&np_exec_mint)?;
    let sk_bin_exec_mint = to_binary(&sk_exec_mint)?;

    let np_cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Execute {
        contract_addr: np_faucet_addr.to_string(),
        funds: vec![],
        msg: np_bin_exec_mint,
    });

    let sk_cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Execute {
        contract_addr: sk_faucet_addr.to_string(),
        funds: vec![],
        msg: sk_bin_exec_mint,
    });

    let np_sub_msg = SubMsg {
        id: 3,
        msg: np_cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    let sk_sub_msg = SubMsg {
        id: 4,
        msg: sk_cosmos_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };
    
    //let bin_prev_count = to_binary(&count)?;

    Ok(Response::new()
        .add_attribute("Mint two NFTs from hub", "minnit")
        .add_submessage(np_sub_msg)
        .add_submessage(sk_sub_msg)
    )
}

pub fn execute_hit_faucet_cw20s(
    deps: Deps,
    info: MessageInfo,
    _env: Env,
) -> Result<Response, ContractError> {

    // pull faucet address' from config
    let config = CONFIGURATION.load(deps.storage)?;
    let cs_one_faucet = config.cw20_one_faucet_addy.ok_or(ContractError::NoFaucetAddy{})?;
    let cs_two_faucet = config.cw20_two_faucet_addy.ok_or(ContractError::NoFaucetAddy{})?;
    let cs_tre_faucet = config.cw20_tre_faucet_addy.ok_or(ContractError::NoFaucetAddy{})?;

    // check that message contains enough ujunox or is admin
    let cost_juno = coin(5000000, "ujunox");
    if !has_coins(&info.funds, &cost_juno) && info.sender != config.admin {
        return Err(ContractError::Unauthorized{});
    }

    // Create mint msgs to sent to cw20 faucet addresses
    let csone_mintmsg = Cw20ExecuteMsg::Mint {
        amount: Uint128::from(69000000u128),
        recipient: info.sender.to_string(),
    };
    let cstwo_mintmsg = Cw20ExecuteMsg::Mint {
        amount: Uint128::from(69000000u128),
        recipient: info.sender.to_string(),
    };
    let cstre_mintmsg = Cw20ExecuteMsg::Mint {
        amount: Uint128::from(69000000u128),
        recipient: info.sender.to_string(),
    };

    let bin_csone = to_binary(&csone_mintmsg)?;
    let bin_cstwo = to_binary(&cstwo_mintmsg)?;
    let bin_cstre = to_binary(&cstre_mintmsg)?;

    let csone_cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Execute {
        contract_addr: cs_one_faucet.to_string(),
        funds: vec![],
        msg: bin_csone,
    });
    let cstwo_cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Execute {
        contract_addr: cs_two_faucet.to_string(),
        funds: vec![],
        msg: bin_cstwo,
    });
    let cstre_cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Execute {
        contract_addr: cs_tre_faucet.to_string(),
        funds: vec![],
        msg: bin_cstre,
    });

    Ok(Response::new()
        .add_attribute("Hit cs20 faucets", "CSONE CSTWO CSTRE")
        .add_message(csone_cosmos_msg)
        .add_message(cstwo_cosmos_msg)
        .add_message(cstre_cosmos_msg)
    )
}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Reply
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

        // Reply from CS-One Contract Init
        11 => {
            let res = parse_reply_instantiate_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "CSONE_Init_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "CSONE_Init_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "CSONE_Init_BrokenUtf8".to_string(), v: "n/e".to_string()}
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
                .add_attribute("Init CS-One Reply Success", format!("Faucet: {}", child_contract.to_string()))
            );

        },

        // Reply from CS-Two Contract Init
        12 => {
            let res = parse_reply_instantiate_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "CSTWO_Init_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "CSTWO_Init_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "CSTWO_Init_BrokenUtf8".to_string(), v: "n/e".to_string()}
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
                .add_attribute("Init CS-Two Reply Success", format!("Faucet: {}", child_contract.to_string()))
            );

        },

        // Reply from CS-Tre contract Init
        13 => {
            let res = parse_reply_instantiate_data(msg.clone())
            .map_err(|e| -> ContractError {
                match e {
                    ParseReplyError::SubMsgFailure(x) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "CSTRE_Init_SubMsgFailure".to_string(), v: x}
                    },
                    ParseReplyError::ParseFailure(y) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "CSTRE_Init_ParseFailure".to_string(), v: y}
                    },
                    ParseReplyError::BrokenUtf8(_z) => {
                        ContractError::SubMsgReplyFailure{p_r_e: "CSTRE_Init_BrokenUtf8".to_string(), v: "n/e".to_string()}
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
                .add_attribute("Init CS-Tre Reply Success", format!("Faucet: {}", child_contract.to_string()))
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

    to_binary(&GetStateResponse {
        admin,
        neon_peepz_address,
        shitty_kittyz_address,
        cw20_one_faucet_address,
        cw20_two_faucet_address,
        cw20_tre_faucet_address,
        neon_peepz_count,
        shitty_kittyz_count,
    })

}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Tests
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {

    //use cosmwasm_std::entry_point;
    //use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
    //use cw2::set_contract_version;
    //use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    //use crate::state::{Config, CONFIG};
    //use crate::error::ContractError;
    //use crate::msg::AdminResponse;
    //use crate::state::{Listing};
    //use cw20::{Balance, Cw20Coin, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg};
    //use crate::msg::{CreateListingMsg};
    //use crate::state::*;
    //use crate::msg::*;

    #[test]
    fn test1() {
        let a = true;
        assert_eq!(a, true);
    }
}