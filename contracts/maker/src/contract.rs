use std::cmp::min;

use cosmwasm_std::{
    generic_err, to_binary, to_vec, unauthorized, Api, Binary, Coin, Env, Extern, HandleResponse,
    InitResponse, Querier, QueryRequest, StdResult, Storage, Uint128,
};
use bitbadges_bindings::{BadgeModuleMsg, BitbadgesMsg, BitbadgesQuerier, BitbadgesQuery};

use crate::msg::{
    // ConfigResponse, ExchangeRateResponse,
    HandleMsg,
    InitMsg,
    // QueryMsg, SimulateResponse,
};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        addresses_to_register: msg.addresses_to_register,
        owner: env.message.sender,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse<BitbadgesMsg>> {
    match msg {
        HandleMsg::Register {  } => register(deps, env),
    }
}

pub fn register<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env
) -> StdResult<HandleResponse<BitbadgesMsg>> {
    let state = config_read(&deps.storage).load()?;
    if env.message.sender != state.owner {
        return Err(unauthorized());
    }

    let contract_addr = deps.api.human_address(&env.contract.address)?;
    // let mut offer = deps.querier.query_balance(&contract_addr, &state.offer)?;
    // if offer.amount == Uint128(0) {
    //     return Ok(HandleResponse::default());
    // }
    // if let Some(stop) = limit {
    //     offer.amount = min(offer.amount, stop);
    // }

    Ok(HandleResponse {
        messages: vec![BadgeModuleMsg::RegisterAddresses {
            addresses_to_register: state.addresses_to_register,
        }
        .into()],
        log: vec![],
        data: None,
    })
}

// pub fn query<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     msg: QueryMsg,
// ) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::Config {} => query_config(deps),
//         QueryMsg::ExchangeRate {} => query_rate(deps),
//         QueryMsg::Simulate { offer } => query_simulate(deps, offer),
//         QueryMsg::Reflect { query } => query_reflect(deps, query),
//     }
// }

// fn query_config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
//     let state = config_read(&deps.storage).load()?;
//     let resp = ConfigResponse {
//         ask: state.ask,
//         offer: state.offer,
//         owner: deps.api.human_address(&state.owner)?,
//     };
//     to_binary(&resp)
// }

// fn query_rate<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Binary> {
//     let state = config_read(&deps.storage).load()?;
//     let rate = BitbadgesQuerier::new(&deps.querier).query_exchange_rate(&state.offer, &state.ask)?;
//     let resp = ExchangeRateResponse {
//         rate,
//         ask: state.ask,
//         offer: state.offer,
//     };
//     to_binary(&resp)
// }

// fn query_simulate<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     offer: Coin,
// ) -> StdResult<Binary> {
//     let state = config_read(&deps.storage).load()?;
//     let ask = if offer.denom == state.ask {
//         state.offer
//     } else if offer.denom == state.offer {
//         state.ask
//     } else {
//         return Err(generic_err(format!(
//             "Cannot simulate '{}' swap, neither contract's ask nor offer",
//             offer.denom
//         )));
//     };
//     let receive = BitbadgesQuerier::new(&deps.querier).query_simulate_swap(offer.clone(), ask)?;
//     let resp = SimulateResponse {
//         sell: offer,
//         buy: receive,
//     };
//     to_binary(&resp)
// }

// fn query_reflect<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     query: BitbadgesQuery,
// ) -> StdResult<Binary> {
//     let request: QueryRequest<BitbadgesQuery> = query.into();
//     let raw_request = to_vec(&request)?;
//     deps.querier
//         .raw_query(&raw_request)
//         .map_err(|e| generic_err(format!("System error: {}", e)))?
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::msg::ConfigResponse;
//     use cosmwasm_std::testing::mock_env;
//     use cosmwasm_std::{coin, coins, from_binary, CosmosMsg, Decimal, HumanAddr, StdError};

//     use bitbadges_bindings::{
//         ExchangeRateResponse as BitbadgesExchangeRateResponse, ExchangeRatesResponse, OracleQuery,
//         RewardsWeightResponse, SeigniorageProceedsResponse, TaxCapResponse, TaxProceedsResponse,
//         TaxRateResponse, TreasuryQuery,
//     };
//     use bitbadges_mocks::mock_dependencies;

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies(20, &[]);

//         let msg = InitMsg {
//             ask: "BTC".into(),
//             offer: "ETH".into(),
//         };
//         let env = mock_env(&deps.api, "creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = init(&mut deps, env, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(&deps, QueryMsg::Config {}).unwrap();
//         let value: ConfigResponse = from_binary(&res).unwrap();
//         assert_eq!("BTC", value.ask.as_str());
//         assert_eq!("ETH", value.offer.as_str());
//         assert_eq!("creator", value.owner.as_str());
//     }

//     #[test]
//     fn buy_limit() {
//         let mut deps = mock_dependencies(20, &coins(200, "ETH"));

//         let msg = InitMsg {
//             ask: "BTC".into(),
//             offer: "ETH".into(),
//         };
//         let env = mock_env(&deps.api, "creator", &coins(200, "ETH"));
//         let _res = init(&mut deps, env, msg).unwrap();

//         // we buy BTC with half the ETH
//         let env = mock_env(&deps.api, "creator", &[]);
//         let contract_addr = deps.api.human_address(&env.contract.address).unwrap();
//         let msg = HandleMsg::Buy {
//             limit: Some(Uint128(100)),
//         };
//         let res = handle(&mut deps, env, msg).unwrap();

//         // make sure we produce proper trade order
//         assert_eq!(1, res.messages.len());
//         if let CosmosMsg::Custom(BitbadgesMsg::Swap(BadgeModuleMsg::Trade {
//             trader_addr,
//             offer_coin,
//             ask_denom,
//         })) = &res.messages[0]
//         {
//             assert_eq!(trader_addr, &contract_addr);
//             assert_eq!(offer_coin, &coin(100, "ETH"));
//             assert_eq!(ask_denom, "BTC");
//         } else {
//             panic!("Expected swap message, got: {:?}", &res.messages[0]);
//         }
//     }

//     #[test]
//     fn only_owner_can_buy() {
//         let mut deps = mock_dependencies(20, &coins(200, "ETH"));

//         let msg = InitMsg {
//             ask: "BTC".into(),
//             offer: "ETH".into(),
//         };
//         let env = mock_env(&deps.api, "creator", &coins(200, "ETH"));
//         let _res = init(&mut deps, env, msg).unwrap();

//         // we buy BTC with half the ETH
//         let env = mock_env(&deps.api, "someone else", &[]);
//         let msg = HandleMsg::Buy {
//             limit: Some(Uint128(100)),
//         };
//         match handle(&mut deps, env, msg).unwrap_err() {
//             StdError::Unauthorized { .. } => {}
//             e => panic!("Expected unauthorized error, got: {}", e),
//         }
//     }

//     #[test]
//     fn sell_no_limit() {
//         let mut deps = mock_dependencies(20, &[coin(200, "ETH"), coin(120, "BTC")]);

//         let msg = InitMsg {
//             ask: "BTC".into(),
//             offer: "ETH".into(),
//         };
//         let env = mock_env(&deps.api, "creator", &[]);
//         let _res = init(&mut deps, env, msg).unwrap();

//         // we sell all the BTC (faked balance above)
//         let env = mock_env(&deps.api, "creator", &[]);
//         let contract_addr = deps.api.human_address(&env.contract.address).unwrap();
//         let msg = HandleMsg::Sell { limit: None };
//         let res = handle(&mut deps, env, msg).unwrap();

//         // make sure we produce proper trade order
//         assert_eq!(1, res.messages.len());
//         if let CosmosMsg::Custom(BitbadgesMsg::Swap(BadgeModuleMsg::Trade {
//             trader_addr,
//             offer_coin,
//             ask_denom,
//         })) = &res.messages[0]
//         {
//             assert_eq!(trader_addr, &contract_addr);
//             assert_eq!(offer_coin, &coin(120, "BTC"));
//             assert_eq!(ask_denom, "ETH");
//         } else {
//             panic!("Expected swap message, got: {:?}", &res.messages[0]);
//         }
//     }

//     #[test]
//     fn sell_limit_higher_than_balance() {
//         let mut deps = mock_dependencies(20, &[coin(200, "ETH"), coin(133, "BTC")]);

//         let msg = InitMsg {
//             ask: "BTC".into(),
//             offer: "ETH".into(),
//         };
//         let env = mock_env(&deps.api, "creator", &[]);
//         let _res = init(&mut deps, env, msg).unwrap();

//         // we sell all the BTC (faked balance above)
//         let env = mock_env(&deps.api, "creator", &[]);
//         let contract_addr = deps.api.human_address(&env.contract.address).unwrap();
//         let msg = HandleMsg::Sell {
//             limit: Some(Uint128(250)),
//         };
//         let res = handle(&mut deps, env, msg).unwrap();

//         // make sure we produce proper trade order
//         assert_eq!(1, res.messages.len());
//         if let CosmosMsg::Custom(BitbadgesMsg::Swap(BadgeModuleMsg::Trade {
//             trader_addr,
//             offer_coin,
//             ask_denom,
//         })) = &res.messages[0]
//         {
//             assert_eq!(trader_addr, &contract_addr);
//             assert_eq!(offer_coin, &coin(133, "BTC"));
//             assert_eq!(ask_denom, "ETH");
//         } else {
//             panic!("Expected swap message, got: {:?}", &res.messages[0]);
//         }
//     }

//     #[test]
//     fn basic_queries() {
//         let mut deps = mock_dependencies(20, &[]);
//         // set the exchange rates between ETH and BTC (and back)
//         deps.querier.with_market(
//             &[
//                 ("ETH", "BTC", Decimal::percent(15)),
//                 ("BTC", "ETH", Decimal::percent(666)),
//             ],
//             &[],
//         );

//         let msg = InitMsg {
//             ask: "BTC".into(),
//             offer: "ETH".into(),
//         };
//         let env = mock_env(&deps.api, "creator", &[]);
//         let _res = init(&mut deps, env, msg).unwrap();

//         // check the config
//         let res = query(&mut deps, QueryMsg::Config {}).unwrap();
//         let cfg: ConfigResponse = from_binary(&res).unwrap();
//         assert_eq!(
//             cfg,
//             ConfigResponse {
//                 owner: HumanAddr::from("creator"),
//                 ask: "BTC".to_string(),
//                 offer: "ETH".to_string(),
//             }
//         );

//         // check the expected rate
//         let res = query(&mut deps, QueryMsg::ExchangeRate {}).unwrap();
//         let cfg: ExchangeRateResponse = from_binary(&res).unwrap();
//         assert_eq!(
//             cfg,
//             ExchangeRateResponse {
//                 ask: "BTC".to_string(),
//                 offer: "ETH".to_string(),
//                 rate: Decimal::percent(15),
//             }
//         );

//         // simulate a forward swap
//         let res = query(
//             &mut deps,
//             QueryMsg::Simulate {
//                 offer: coin(100, "ETH"),
//             },
//         )
//         .unwrap();
//         let cfg: SimulateResponse = from_binary(&res).unwrap();
//         assert_eq!(
//             cfg,
//             SimulateResponse {
//                 sell: coin(100, "ETH"),
//                 buy: coin(15, "BTC"),
//             }
//         );

//         // simulate a reverse swap
//         let res = query(
//             &mut deps,
//             QueryMsg::Simulate {
//                 offer: coin(10, "BTC"),
//             },
//         )
//         .unwrap();
//         let cfg: SimulateResponse = from_binary(&res).unwrap();
//         assert_eq!(
//             cfg,
//             SimulateResponse {
//                 sell: coin(10, "BTC"),
//                 buy: coin(66, "ETH"),
//             }
//         );
//     }

//     #[test]
//     fn query_exchange_rates() {
//         let mut deps = mock_dependencies(20, &[]);
//         // set the exchange rates between ETH and BTC (and back)
//         deps.querier.with_market(
//             &[
//                 ("ETH", "BTC", Decimal::percent(15)),
//                 ("BTC", "ETH", Decimal::percent(666)),
//                 ("ETH", "ATOM", Decimal::percent(1234)),
//             ],
//             &[],
//         );

//         let msg = InitMsg {
//             ask: "BTC".into(),
//             offer: "ETH".into(),
//         };
//         let env = mock_env(&deps.api, "creator", &[]);
//         let _res = init(&mut deps, env, msg).unwrap();

//         // check the general exchange query
//         let rates_query = BitbadgesQuery::Oracle(OracleQuery::ExchangeRates {
//             offer: "ETH".to_string(),
//         });
//         let res = query(&mut deps, QueryMsg::Reflect { query: rates_query }).unwrap();
//         let rates: ExchangeRatesResponse = from_binary(&res).unwrap();
//         assert_eq!(2, rates.rates.len());
//         assert_eq!(
//             rates.rates[0],
//             BitbadgesExchangeRateResponse {
//                 rate: Decimal::percent(1234),
//                 ask: "ATOM".to_string(),
//             }
//         );
//         assert_eq!(
//             rates.rates[1],
//             BitbadgesExchangeRateResponse {
//                 rate: Decimal::percent(15),
//                 ask: "BTC".to_string(),
//             }
//         );
//     }

//     #[test]
//     fn query_treasury() {
//         let mut deps = mock_dependencies(20, &[]);
//         // set the exchange rates between ETH and BTC (and back)
//         let tax_rate = Decimal::percent(2);
//         let tax_proceeds = vec![coin(10, "ETH"), coin(20, "BTC")];
//         let tax_caps = &[("ETH", 1000u128), ("BTC", 500u128)];
//         let reward = Decimal::permille(5);
//         let seignorage = 777;

//         deps.querier
//             .with_treasury(tax_rate, &tax_proceeds, tax_caps, reward, seignorage);

//         let msg = InitMsg {
//             ask: "BTC".into(),
//             offer: "ETH".into(),
//         };
//         let env = mock_env(&deps.api, "creator", &[]);
//         let _res = init(&mut deps, env, msg).unwrap();

//         // test all treasury functions
//         let tax_rate_query = QueryMsg::Reflect {
//             query: TreasuryQuery::TaxRate {}.into(),
//         };
//         let res = query(&mut deps, tax_rate_query).unwrap();
//         let rate: TaxRateResponse = from_binary(&res).unwrap();
//         assert_eq!(rate.tax, tax_rate);

//         let tax_cap_query = QueryMsg::Reflect {
//             query: TreasuryQuery::TaxCap {
//                 denom: "ETH".to_string(),
//             }
//             .into(),
//         };
//         let res = query(&mut deps, tax_cap_query).unwrap();
//         let cap: TaxCapResponse = from_binary(&res).unwrap();
//         assert_eq!(cap.cap, Uint128(1000));

//         let tax_proceeds_query = QueryMsg::Reflect {
//             query: TreasuryQuery::TaxProceeds {}.into(),
//         };
//         let res = query(&mut deps, tax_proceeds_query).unwrap();
//         let proceeds: TaxProceedsResponse = from_binary(&res).unwrap();
//         assert_eq!(proceeds.proceeds, tax_proceeds);

//         let rewards_query = QueryMsg::Reflect {
//             query: TreasuryQuery::RewardsWeight {}.into(),
//         };
//         let res = query(&mut deps, rewards_query).unwrap();
//         let rewards: RewardsWeightResponse = from_binary(&res).unwrap();
//         assert_eq!(rewards.weight, reward);

//         let seigniorage_query = QueryMsg::Reflect {
//             query: TreasuryQuery::SeigniorageProceeds {}.into(),
//         };
//         let res = query(&mut deps, seigniorage_query).unwrap();
//         let proceeds: SeigniorageProceedsResponse = from_binary(&res).unwrap();
//         assert_eq!(proceeds.size, Uint128(seignorage));
//     }
// }
