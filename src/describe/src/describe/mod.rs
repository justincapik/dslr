mod compute;
mod present;

use polars::prelude::{DataFrame, PolarsResult};

use crate::Args;

pub fn describe(df: DataFrame, args: &Args) -> PolarsResult<()> {
	lazy_frame(df.clone());

	let table = compute::compute(df, args)?;
	present::present(table, args)
}

use polars::prelude::*;

fn lazy_frame(df: DataFrame) {
	dbg!(df.clone().lazy().select(&[sum("*")]).collect().unwrap());
}
