use polars::prelude::*;
use tabled::{builder::Builder, Table};

use crate::Args;

struct Analyze {
	name: String,
	dtype: DataType,

	min: Option<f64>,
	max: Option<f64>,
	mean: Option<f64>,
	median: Option<f64>,
	sum: Option<f64>,
}

type TableRecord<'s> = [&'s str; 7];

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
		let sum = &to_string(series.sum_reduce()?.value().try_extract::<f64>().ok(), args);

		builder.push_record([name, dtype, min, max, mean, median, sum]);
	}

	Ok((builder.build(), types))
}

impl From<&Series> for Analyze {
	fn from(series: &Series) -> Self {
		let name = series.name().to_owned();
		let dtype = series.dtype().to_owned();

		let mut min = None;
		let mut max = None;
		let mut mean = None;
		let mut median = None;
		let mut sum = None;

		for (i, value) in series.into_iter().enumerate() {
			let value = value?;
			if i == 0 {
				min = Some(value);
				max = Some(value);
				mean = Some(value);
				median = Some(value);
				sum = Some(value);
			} else {
				min = min.map(|min| min.min(value));
				max = max.map(|max| max.max(value));
				mean = mean.map(|mean| mean + value);
				median = median.map(|median| median + value);
				sum = sum.map(|sum| sum + value);
			}
		}

		Self {
			name: series.name().to_owned(),
			dtype: series.dtype().to_owned(),

			min: series.min().unwrap(),
			max: series.max().unwrap(),
			mean: series.mean(),
			median: series.median(),
			sum: series
				.sum_reduce()
				.unwrap()
				.value()
				.try_extract::<f64>()
				.ok(),
		}
	}
}

impl Analyze {
	fn headers() -> TableRecord<'static> {
		["column", "type", "min", "max", "mean", "median", "sum"]
	}
}

fn to_string(n: Option<f64>, args: &Args) -> String {
	let Some(n) = n else {
		return String::new();
	};

	format!("{:.1$}", n, args.round as usize)
}
