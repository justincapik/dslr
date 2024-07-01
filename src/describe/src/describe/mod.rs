mod compute;
mod present;

use polars::prelude::{DataFrame, PolarsResult};

use crate::Args;

pub fn describe(df: DataFrame, args: &Args) -> PolarsResult<()> {
	let table = compute::compute(df, args)?;
	present::present(table, args)
}
