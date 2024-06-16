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

		let mut min: Option<f64> = None;
		let mut max: Option<f64> = None;
		let mut sum: Option<f64> = None;

		if dtype.is_numeric() {
			for chunk in series.f64() {
				dbg!(chunk);

				let mut first = true;
				for value in chunk {
					let Some(value) = value else {
						continue;
					};

					if first {
						min = Some(value);
						max = Some(value);
						sum = Some(value);

						first = false;
					} else {
						min = min.map(|min| min.min(value));
						max = max.map(|max| max.max(value));
						sum = sum.map(|sum| sum + value);
					}
				}
			}
		}

		Self {
			name,
			dtype,

			min,
			max,
			mean: series.mean(),
			median: series.median(),
			sum,
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_analyze_basic() {
		let s = Series::new("a", &[1.0, 2.0, 3.0, 4.0, 5.0]);
		let a = Analyze::from(&s);

		assert_eq!(a.min, Some(1.0));
		assert_eq!(a.min, s.min().unwrap());
		assert_eq!(a.max, Some(5.0));
		assert_eq!(a.mean, Some(3.0));
		assert_eq!(a.median, Some(3.0));
		assert_eq!(a.sum, Some(15.0));
	}

	#[test]
	fn test_analyze_negative() {
		let s = Series::new("a", &[-42.0, -5.0, 0.0, 1.0, 2.0, 1001.0]);
		let a = Analyze::from(&s);

		assert_eq!(a.min, Some(-42.0));
		assert_eq!(a.max, Some(1001.0));
		assert_eq!(a.mean, Some(159.5));
		assert_eq!(a.median, Some(0.5));
		assert_eq!(a.sum, Some(957.0));
	}

	#[test]
	fn test_analyze_empty() {
		let empty: [f64; 0] = [];
		let s = Series::new("a", &empty);
		let a = Analyze::from(&s);

		assert_eq!(a.min, None);
		assert_eq!(a.max, None);
		assert_eq!(a.mean, None);
		assert_eq!(a.median, None);
		assert_eq!(a.sum, None);
	}

	#[test]
	fn test_analyze_str() {
		let s = Series::new("a", &["a", "b", "c"]);
		let a = Analyze::from(&s);

		assert_eq!(a.min, None);
		assert_eq!(a.max, None);
		assert_eq!(a.mean, None);
		assert_eq!(a.median, None);
		assert_eq!(a.sum, None);
	}
}
