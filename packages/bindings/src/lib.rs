mod msg;
mod querier;
mod query;

pub use msg::{BadgeModuleMsg, BitbadgesMsg};
pub use querier::BitbadgesQuerier;
pub use query::{
    ExchangeRateResponse, ExchangeRatesResponse, OracleQuery, RewardsWeightResponse,
    SeigniorageProceedsResponse, SimulateSwapResponse, SwapQuery, TaxCapResponse,
    TaxProceedsResponse, TaxRateResponse, BitbadgesQuery, TobinTaxResponse, TreasuryQuery,
};

// This export is added to all contracts that import this package, signifying that they require
// "bitbadges" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_bitbadges() {}
