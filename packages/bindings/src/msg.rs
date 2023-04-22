use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    // Coin, 
    CosmosMsg, 
    HumanAddr
};

/// BitbadgesMsg is an override of CosmosMsg::Custom to add support for Bitbadges's custom message types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BitbadgesMsg {
    BadgeModule(BadgeModuleMsg),
}

// BadgeModuleMsg captures all possible messages we can return to bitbadges's native badges module
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum BadgeModuleMsg {
    RegisterAddresses {
        addresses_to_register: Vec<String>,
    },
}

// this is a helper to be able to return these as CosmosMsg easier
impl Into<CosmosMsg<BitbadgesMsg>> for BitbadgesMsg {
    fn into(self) -> CosmosMsg<BitbadgesMsg> {
        CosmosMsg::Custom(self)
    }
}

// and another helper, so we can return BadgeModuleMsg::RegisterAddresses{..}.into() as a CosmosMsg
impl Into<CosmosMsg<BitbadgesMsg>> for BadgeModuleMsg {
    fn into(self) -> CosmosMsg<BitbadgesMsg> {
        CosmosMsg::Custom(BitbadgesMsg::BadgeModule(self))
    }
}
