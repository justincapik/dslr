use polars::prelude::*;
use tabled::{builder::Builder, Table};

use crate::Args;

pub fn compute(df: DataFrame, args: &Args) -> PolarsResult<(Table, Vec<DataType>)> {
	let mut types = Vec::with_capacity(df.width());

	let mut builder = Builder::default();

	builder.push_record(["column", "type", "min", "max", "mean", "median", "sum"]);

	for series in df.get_columns() {
		let name = series.name();

		let dtype = series.dtype();
		types.push(dtype.clone());
		let dtype = &dtype.to_string();

		let min = &to_string(series.min()?, args);
		let max = &to_string(series.max()?, args);
		let mean = &to_string(series.mean(), args);
		// let std = series.std();
		let median = &to_string(series.median(), args);
		// let q1 = series.quantile(0.25)?;
		// let q3 = series.quantile(0.75)?;
		// let sum = &to_string(series.sum_reduce()?
		let sum = &to_string(
			series
				.sum_reduce()?
				.value()
				.try_extract::<f64>().ok(),
			args,
		);

		builder.push_record([name, dtype, min, max, mean, median, sum]);
	}

	Ok((builder.build(), types))
}

fn to_string(n: Option<f64>, args: &Args) -> String {
	let Some(n) = n else {
		return String::new();
	};

	format!("{:.1$}", n, args.round as usize)
}
