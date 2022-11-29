use cosmwasm_std::{Addr, SubMsg, to_binary, CosmosMsg, Empty, WasmMsg, ReplyOn};
use nois::{ints_in_range, int_in_range}; // sub_randomness_with_key
use crate::error::*;

use cw721_neon_peepz::{
    ExecuteMsg as Cw721NeonPeepzExecuteMsg,
    MintMsg as NeonPeepzMintMsg,
    NeonPeepzExtension,
    RandomNP,
};

use cw721_shitty_kittyz::{
    ExecuteMsg as Cw721ShittyKittyzExecuteMsg,
    MintMsg as ShittyKittyzMintMsg,
    ShittyKittyzExtension,
    RandomSK,
};

pub fn make_mint_submsgs(
    np_faucet_addr: Addr,
    np_count: u32,
    sk_faucet_addr: Addr,
    sk_count: u32,
    user_wallet: Addr,
    randomness: [u8; 32],
) -> Result<Vec<SubMsg>, ContractError> {

    // Shitty Kittyz only has 1 arbritary trait: Fur Color, which is 1 of 3 options
    let sktrait = int_in_range(randomness, 1u32..=3u32);

    // Neon Peepz have 2 arbitrary traits, so we get 2 rand numbers
    let [nptrait1, nptrait2] = ints_in_range(randomness, 1u32..=2u32);

    // Legend Catch numbers
    let [sk_legend_catch, np_legend_catch] = ints_in_range(randomness, 1u32..=333u32);

    let np_metadata = generate_np_metadata(
        np_legend_catch, 
        nptrait1,
        nptrait2
    );

    let sk_metadata = generate_sk_metadata(
        sk_legend_catch,
        sktrait,
    );

    let submsgs = construct_submsgs(
        np_faucet_addr, 
        np_count, 
        np_metadata, 
        sk_faucet_addr, 
        sk_count, 
        sk_metadata, 
        user_wallet
    )?;

    Ok(submsgs)
}

pub fn generate_sk_metadata(
    legend_check_num: u32,
    furcolortrait: u32,
) -> ShittyKittyzExtension {

    if legend_check_num != 333 {
        // Call function that generates random common SK metadata
        return ShittyKittyzExtension::rand_common_sk_metadata(furcolortrait)
    } else {
        // Return the 1 single Legendary Shitty Kitty Metadata
        return ShittyKittyzExtension::legendary_sk_metadata()
    }

}

pub fn generate_np_metadata(
    legend_check_num: u32,
    nptrait1: u32,
    nptrait2: u32
) -> NeonPeepzExtension {

    match legend_check_num {
        111 => {
            NeonPeepzExtension::bot_legend()
        },
        222 => {
            NeonPeepzExtension::fairy_legend()
        },
        333 => {
            NeonPeepzExtension::denom_legend()
        },
        _ => {
            NeonPeepzExtension::rand_peep(nptrait1, nptrait2)
        }
    }

}

pub fn construct_submsgs(
    np_faucet_addr: Addr,
    np_count: u32,
    np_metadata: NeonPeepzExtension,
    sk_faucet_addr: Addr,
    sk_count: u32,
    sk_metadata: ShittyKittyzExtension,
    user_wallet: Addr,
) -> Result<Vec<SubMsg>, ContractError> {

    let np_mint_msg: NeonPeepzMintMsg<NeonPeepzExtension> = NeonPeepzMintMsg{
        count: np_count,
        token_id: np_count.to_string(),
        owner: user_wallet.to_string(),
        token_uri: None,
        extension: np_metadata
    };

    let sk_mint_msg: ShittyKittyzMintMsg<ShittyKittyzExtension> = ShittyKittyzMintMsg{
        count: sk_count,
        token_id: sk_count.to_string(),
        owner: user_wallet.to_string(),
        token_uri: None,
        extension: sk_metadata
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

    Ok(vec![np_sub_msg, sk_sub_msg])

}