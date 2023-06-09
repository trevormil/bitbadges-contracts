use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use bitbadges_bindings::{
    ExchangeRateResponse, ExchangeRatesResponse, RewardsWeightResponse,
    SeigniorageProceedsResponse, SimulateSwapResponse, TaxCapResponse, TaxProceedsResponse,
    TaxRateResponse, BitbadgesMsg, BitbadgesQuery, TobinTaxResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(BitbadgesMsg), &out_dir);
    export_schema(&schema_for!(BitbadgesQuery), &out_dir);
    export_schema(&schema_for!(ExchangeRateResponse), &out_dir);
    export_schema(&schema_for!(ExchangeRatesResponse), &out_dir);
    export_schema(&schema_for!(RewardsWeightResponse), &out_dir);
    export_schema(&schema_for!(SeigniorageProceedsResponse), &out_dir);
    export_schema(&schema_for!(SimulateSwapResponse), &out_dir);
    export_schema(&schema_for!(TaxCapResponse), &out_dir);
    export_schema(&schema_for!(TaxProceedsResponse), &out_dir);
    export_schema(&schema_for!(TaxRateResponse), &out_dir);
    export_schema(&schema_for!(TobinTaxResponse), &out_dir);
}
