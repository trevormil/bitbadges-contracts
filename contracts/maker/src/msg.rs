use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Decimal, HumanAddr, Uint128};

use bitbadges_bindings::BitbadgesQuery;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub addresses_to_register: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Register {  },
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     /// Config returns the stored configuration state. Returns State
//     Config {},
//     /// Exchange rate returns how many ASK we can get for 1 OFFER
//     ExchangeRate {},
//     /// Simulate will try to sell the given number of tokens (denom must be either ask or offer, we trade for the other)
//     Simulate { offer: Coin },
//     /// Reflect is used for developer integration tests on the go layer.
//     /// This will cause the contract to make this query (which goes to the SDK), then return the result
//     /// to the user. This can be used to test the query handlers full-stack in Go code.
//     ///
//     /// There are many possible return values here, this will just return the raw bytes, the caller
//     /// is required to know the proper response type (defined in bitbadges_bindings)
//     Reflect { query: BitbadgesQuery },
// }

// /// Returns rate of ASK/OFFER
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct ExchangeRateResponse {
//     pub rate: Decimal,
//     pub ask: String,
//     pub offer: String,
// }

// /// Returns how many coins we could BUY if we SELL the given amount
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct SimulateResponse {
//     pub sell: Coin,
//     pub buy: Coin,
// }

// /// Human readable state
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct ConfigResponse {
//     pub offer: String,
//     pub ask: String,
//     pub owner: HumanAddr,
// }
