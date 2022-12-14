use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw2::set_contract_version;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cs_faucet_neonpeepz";
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

pub type NeonPeepzExtension = Option<Metadata>;

pub trait RandomNP {
    fn bot_legend() -> NeonPeepzExtension;

    fn fairy_legend() -> NeonPeepzExtension;

    fn denom_legend() -> NeonPeepzExtension;

    fn rand_peep(trait1: u32, trait2: u32) -> NeonPeepzExtension;
}

impl RandomNP for NeonPeepzExtension {

    fn bot_legend() -> NeonPeepzExtension {
        let name = "Legendary Bot".to_string();
        let image_link = "https://bafybeihpieunve4hqyohwcqx7l3sxrm6b6b2m2rzn4r2h62fxrecek64pi.ipfs.nftstorage.link/NeoNPeePz/legendary_bot.png".to_string();
        let (arch_trait, class_trait, weapon_trait) = (
            Trait {display_type: None, trait_type: "Archetype".to_string(), value: "Deity".to_string()},
            Trait {display_type: None, trait_type: "Class".to_string(), value: "Robotic".to_string()},
            Trait {display_type: None, trait_type: "Weapon".to_string(), value: "Laser Face".to_string()}
        );
        return Some(Metadata {
            name: Some(name),
            image: Some(image_link),
            description: Some("NeoNPeePz Collection".to_string()),
            attributes: Some(vec![arch_trait, class_trait, weapon_trait]),
            ..Metadata::default()
        });
    }

    fn fairy_legend() -> NeonPeepzExtension {
        let name = "Legendary Fairy".to_string();
        let image_link = "https://bafybeihpieunve4hqyohwcqx7l3sxrm6b6b2m2rzn4r2h62fxrecek64pi.ipfs.nftstorage.link/NeoNPeePz/legendary_fairy.png".to_string();
        let (arch_trait, class_trait, weapon_trait) = (
            Trait {display_type: None, trait_type: "Archetype".to_string(), value: "Deity".to_string()},
            Trait {display_type: None, trait_type: "Class".to_string(), value: "Celestial".to_string()},
            Trait {display_type: None, trait_type: "Weapon".to_string(), value: "Arcane Bolt".to_string()}
        );
        return Some(Metadata {
            name: Some(name),
            image: Some(image_link),
            description: Some("NeoNPeePz Collection".to_string()),
            attributes: Some(vec![arch_trait, class_trait, weapon_trait]),
            ..Metadata::default()
        });
        
    }

    fn denom_legend() -> NeonPeepzExtension {
        let name = "Legendary Demon".to_string();
        let image_link = "https://bafybeihpieunve4hqyohwcqx7l3sxrm6b6b2m2rzn4r2h62fxrecek64pi.ipfs.nftstorage.link/NeoNPeePz/legendary_demon.png".to_string();
        let (arch_trait, class_trait, weapon_trait) = (
            Trait {display_type: None, trait_type: "Archetype".to_string(), value: "Deity".to_string()},
            Trait {display_type: None, trait_type: "Class".to_string(), value: "Cursed".to_string()},
            Trait {display_type: None, trait_type: "Weapon".to_string(), value: "Drain Life".to_string()}
        );
        return Some(Metadata {
            name: Some(name),
            image: Some(image_link),
            description: Some("NeoNPeePz Collection".to_string()),
            attributes: Some(vec![arch_trait, class_trait, weapon_trait]),
            ..Metadata::default()
        });
    }


    fn rand_peep(trait1: u32, trait2: u32) -> NeonPeepzExtension {

        let (name, image_link, arch_trait, class_trait, weapon_trait) = match trait1 {

            1 => { // "Mage"
                match trait2 {
                    1 => { // "Hero" - Hero Mage
                        let name = "Hero-Mage".to_string();
                        let image_link = "https://bafybeihpieunve4hqyohwcqx7l3sxrm6b6b2m2rzn4r2h62fxrecek64pi.ipfs.nftstorage.link/NeoNPeePz/hero_mage.png".to_string();
                        let arch_trait = Trait {display_type: None, trait_type: "Archetype".to_string(), value: "Hero".to_string()};
                        let class_trait = Trait {display_type: None, trait_type: "Class".to_string(), value: "Mage".to_string()};
                        let weapon_trait = Trait {display_type: None, trait_type: "Weapon".to_string(), value: "Cracked Staff".to_string()};
                        (name, image_link, arch_trait, class_trait, weapon_trait)
                    },

                    _ => { // "Villain" - Villain Mage
                        let name = "Villain-Mage".to_string();
                        let image_link = "https://bafybeihpieunve4hqyohwcqx7l3sxrm6b6b2m2rzn4r2h62fxrecek64pi.ipfs.nftstorage.link/NeoNPeePz/villian_mage.png".to_string();
                        let arch_trait = Trait {display_type: None, trait_type: "Archetype".to_string(), value: "Villain".to_string()};
                        let class_trait = Trait {display_type: None, trait_type: "Class".to_string(), value: "Mage".to_string()};
                        let weapon_trait = Trait {display_type: None, trait_type: "Weapon".to_string(), value: "Cracked Staff".to_string()};
                        (name, image_link, arch_trait, class_trait, weapon_trait)
                    }
                }
            },

            _ => { // "Warrior"
                match trait2 {

                    1 => { // "Hero" - Hero Warrior
                        let name = "Hero-Warrior".to_string();
                        let image_link = "https://bafybeihpieunve4hqyohwcqx7l3sxrm6b6b2m2rzn4r2h62fxrecek64pi.ipfs.nftstorage.link/NeoNPeePz/hero_warrior.png".to_string();
                        let arch_trait = Trait {display_type: None, trait_type: "Archetype".to_string(), value: "Hero".to_string()};
                        let class_trait = Trait {display_type: None, trait_type: "Class".to_string(), value: "Warrior".to_string()};
                        let weapon_trait = Trait {display_type: None, trait_type: "Weapon".to_string(), value: "Dull Blade".to_string()};
                        (name, image_link, arch_trait, class_trait, weapon_trait)
                    },

                    _ => { // "Villain" - Villain Warrior
                        let name = "Villain-Warrior".to_string();
                        let image_link = "https://bafybeihpieunve4hqyohwcqx7l3sxrm6b6b2m2rzn4r2h62fxrecek64pi.ipfs.nftstorage.link/NeoNPeePz/villian_warrior.png".to_string();
                        let arch_trait = Trait {display_type: None, trait_type: "Archetype".to_string(), value: "Villain".to_string()};
                        let class_trait = Trait {display_type: None, trait_type: "Class".to_string(), value: "Warrior".to_string()};
                        let weapon_trait = Trait {display_type: None, trait_type: "Weapon".to_string(), value: "Dull Blade".to_string()};
                        (name, image_link, arch_trait, class_trait, weapon_trait)
                    }
                }
            }
        };

        Some(Metadata {
            name: Some(name),
            image: Some(image_link),
            description: Some("NeoNPeepz Collection".to_string()),
            attributes: Some(vec![arch_trait, class_trait, weapon_trait]),
            ..Metadata::default()
        })
        
    }

}

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, NeonPeepzExtension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<NeonPeepzExtension, Empty>;
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
