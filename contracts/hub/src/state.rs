use cosmwasm_std::{Addr};
use cw_storage_plus::{
    Item, 
}; //Map, UniqueIndex, IndexList, Index, IndexedMap, MultiIndex

use cosmwasm_schema::cw_serde;

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Config
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub const CONFIGURATION: Item<Configuration> = Item::new("cw721_hub_config");

#[cw_serde]
pub struct Configuration {
    pub admin: Addr,
    pub neon_peepz_addy: Option<Addr>,
    pub shitty_kittyz_addy: Option<Addr>,
    pub cw20_one_faucet_addy: Option<Addr>,
    pub cw20_two_faucet_addy: Option<Addr>,
    pub cw20_tre_faucet_addy: Option<Addr>,
    pub nois_beacon: Addr,
}

pub const NPCOUNT: Item<u32> = Item::new("neon_peepz_count");
pub const SKCOUNT: Item<u32> = Item::new("shitty_kittyz_count");

pub const JOBINCREMENT: Item<u32> = Item::new("job_increment");





