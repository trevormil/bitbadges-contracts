use cosmwasm_std::testing::{
    mock_dependencies as std_dependencies, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{
    from_slice, Coin, Decimal, Extern, FullDelegation, HumanAddr, Querier, QuerierResult,
    QueryRequest, SystemError, Validator,
};

use crate::{OracleQuerier, SwapQuerier, TreasuryQuerier};
use bitbadges_bindings::BitbadgesQuery;

/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn mock_dependencies(
    canonical_length: usize,
    contract_balance: &[Coin],
) -> Extern<MockStorage, MockApi, BitbadgesMockQuerier> {
    let base = std_dependencies(canonical_length, contract_balance);
    base.change_querier(BitbadgesMockQuerier::new)
}

#[derive(Clone, Default)]
pub struct BitbadgesMockQuerier {
    base: MockQuerier,
    swap: SwapQuerier,
    oracle: OracleQuerier,
    treasury: TreasuryQuerier,
}

impl Querier for BitbadgesMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<BitbadgesQuery> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl BitbadgesMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<BitbadgesQuery>) -> QuerierResult {
        match &request {
            QueryRequest::Custom(custom) => match custom {
                BitbadgesQuery::Swap(swap_query) => self.swap.query(swap_query),
                BitbadgesQuery::Oracle(oracle_query) => self.oracle.query(oracle_query),
                BitbadgesQuery::Treasury(treasury_query) => self.treasury.query(treasury_query),
            },
            _ => self.base.handle_query(request),
        }
    }
}

impl BitbadgesMockQuerier {
    pub fn new(base: MockQuerier) -> Self {
        BitbadgesMockQuerier {
            base,
            swap: SwapQuerier::default(),
            oracle: OracleQuerier::default(),
            treasury: TreasuryQuerier::default(),
        }
    }

    // set a new balance for the given address and return the old balance
    pub fn update_balance<U: Into<HumanAddr>>(
        &mut self,
        addr: U,
        balance: Vec<Coin>,
    ) -> Option<Vec<Coin>> {
        self.base.update_balance(addr, balance)
    }

    // configure the stacking mock querier
    pub fn with_staking(
        &mut self,
        denom: &str,
        validators: &[Validator],
        delegations: &[FullDelegation],
    ) {
        self.base.with_staking(denom, validators, delegations)
    }

    pub fn with_market(&mut self, rates: &[(&str, &str, Decimal)], taxes: &[(&str, Decimal)]) {
        self.oracle = OracleQuerier::new(rates, taxes);
        self.swap = SwapQuerier::new(rates);
    }

    pub fn with_treasury(
        &mut self,
        tax_rate: Decimal,
        tax_proceeds: &[Coin],
        tax_caps: &[(&str, u128)],
        reward_rate: Decimal,
        seigniorage_proceeds: u128,
    ) {
        self.treasury = TreasuryQuerier::new(
            tax_rate,
            tax_proceeds,
            tax_caps,
            reward_rate,
            seigniorage_proceeds,
        );
    }
}
