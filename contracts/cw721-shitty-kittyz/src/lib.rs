use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw2::set_contract_version;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cs_faucet_shittykittyz";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub name: Option<String>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub attributes: Option<Vec<Trait>>,

    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

pub type ShittyKittyzExtension = Option<Metadata>;

pub trait RandomSK {
    fn rand_metadata_extension_skittyz(blockheight: u64) -> ShittyKittyzExtension;
}

impl RandomSK for ShittyKittyzExtension {
    fn rand_metadata_extension_skittyz(blockheight: u64) -> ShittyKittyzExtension {

        // timecrunch just make galactic 1/100
        let bh = blockheight.to_string();
        let (_bh_one, bh_two) = bh.split_at(bh.len() - 2);

        let legend_catchnum = bh_two.parse::<u64>();
        if let Err(_e) = legend_catchnum.clone() {
            return ShittyKittyzExtension::default();
        };

        if legend_catchnum.clone().unwrap() == 33 {

            let name = "Interstellar Pooper".to_string();
            let image_link = "https://bafybeig3irsnosywnthdhgfslvmtum7wwkdcatzojji7pjpnamzjoevff4.ipfs.nftstorage.link/ShittyKittyz/sk_galactic.png".to_string();

            let (fur_trait, background_trait, rarity_trait) = (
                Trait {display_type: None, trait_type: "Fur".to_string(), value: "Galactic".to_string()},
                Trait {display_type: None, trait_type: "Background".to_string(), value: "Rainbow".to_string()},
                Trait {display_type: None, trait_type: "Rarity".to_string(), value: "Epic".to_string()}
            );
            return Some(Metadata {
                name: Some(name),
                image: Some(image_link),
                description: Some("ShittyKittyz Collection".to_string()),
                attributes: Some(vec![fur_trait, background_trait, rarity_trait]),
                ..Metadata::default()
            });
        };

        let bhtwo = blockheight.to_string();
        let (_x, numtwo) = bhtwo.split_at(bhtwo.len() - 1);

        let ran = numtwo.parse::<u64>();
        if let Err(_e) = ran.clone() {
            return ShittyKittyzExtension::default();
        };
        let rand = ran.unwrap();

        // Brown
        if (rand == 1) || (rand == 5) || (rand == 8) {
            let name = "Basic Pooper".to_string();
            let image_link = "https://bafybeig3irsnosywnthdhgfslvmtum7wwkdcatzojji7pjpnamzjoevff4.ipfs.nftstorage.link/ShittyKittyz/sk_brown.png".to_string();

            let (fur_trait, background_trait, rarity_trait) = (
                Trait {display_type: None, trait_type: "Fur".to_string(), value: "Brown".to_string()},
                Trait {display_type: None, trait_type: "Background".to_string(), value: "Poo-Green".to_string()},
                Trait {display_type: None, trait_type: "Rarity".to_string(), value: "Common".to_string()}
            );
            return Some(Metadata {
                name: Some(name),
                image: Some(image_link),
                description: Some("ShittyKittyz Collection".to_string()),
                attributes: Some(vec![fur_trait, background_trait, rarity_trait]),
                ..Metadata::default()
            });

        // Gray
        } else if (rand == 2) || (rand == 4) || (rand == 9) || (rand == 0) {
            let name = "Basic Pooper".to_string();
            let image_link = "https://bafybeig3irsnosywnthdhgfslvmtum7wwkdcatzojji7pjpnamzjoevff4.ipfs.nftstorage.link/ShittyKittyz/sk_gray.png".to_string();

            let (fur_trait, background_trait, rarity_trait) = (
                Trait {display_type: None, trait_type: "Fur".to_string(), value: "Gray".to_string()},
                Trait {display_type: None, trait_type: "Background".to_string(), value: "Poo-Green".to_string()},
                Trait {display_type: None, trait_type: "Rarity".to_string(), value: "Common".to_string()}
            );
            return Some(Metadata {
                name: Some(name),
                image: Some(image_link),
                description: Some("ShittyKittyz Collection".to_string()),
                attributes: Some(vec![fur_trait, background_trait, rarity_trait]),
                ..Metadata::default()
            });

        // Orange
        } else {

            let name = "Basic Pooper".to_string();
            let image_link = "https://bafybeig3irsnosywnthdhgfslvmtum7wwkdcatzojji7pjpnamzjoevff4.ipfs.nftstorage.link/ShittyKittyz/sk_orange.png".to_string();

            let (fur_trait, background_trait, rarity_trait) = (
                Trait {display_type: None, trait_type: "Fur".to_string(), value: "Orange".to_string()},
                Trait {display_type: None, trait_type: "Background".to_string(), value: "Poo-Green".to_string()},
                Trait {display_type: None, trait_type: "Rarity".to_string(), value: "Common".to_string()}
            );
            return Some(Metadata {
                name: Some(name),
                image: Some(image_link),
                description: Some("ShittyKittyz Collection".to_string()),
                attributes: Some(vec![fur_trait, background_trait, rarity_trait]),
                ..Metadata::default()
            });
        };

    }
}

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, ShittyKittyzExtension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<ShittyKittyzExtension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {

        let res = Cw721MetadataContract::default().instantiate(deps.branch(), env, info, msg)?;

        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::Cw721Query;

    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = MintMsg {
            count: 33,
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Some(Metadata {
                //description: Some("Spaceship with Warp Drive".into()),
                name: Some("Starship USS Enterprise".to_string()),
                ..Metadata::default()
            }),
        };
        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, mint_msg.token_uri);
        assert_eq!(res.extension, mint_msg.extension);
    }
}
