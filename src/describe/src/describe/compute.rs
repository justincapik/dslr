use polars::prelude::*;
use tabled::{builder::Builder, Table};

use crate::Args;

#[derive(Debug, Clone, PartialEq)]
struct Analyze {
	name: String,
	dtype: DataType,

	min: Option<f64>,
	max: Option<f64>,
	mean: Option<f64>,
	median: Option<f64>,
	// q1
	// q3
	std: Option<f64>,
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

impl From<Series> for Analyze {
	fn from(series: Series) -> Self {
		Analyze::from(&series)
	}
}

impl From<&Series> for Analyze {
	fn from(series: &Series) -> Self {
		let name = series.name().to_owned();
		let dtype = series.dtype().to_owned();

		let mut ret = Self {
			name,
			dtype,

			min: None,
			max: None,
			mean: None,
			median: None,
			std: None,
			sum: None,
		};

		if !ret.dtype.is_numeric() {
			return ret;
		}

		let mut arr = series
			.f64()
			.expect("could not extract series as f64 iterator")
			.into_iter()
			.filter(|x| x.is_some())
			.map(|x| x.unwrap())
			.collect::<Vec<f64>>();

		arr.sort_by(|a, b| a.total_cmp(b));

		let Some(first) = arr.first().copied() else {
			return ret;
		};

		let mut min = first;
		let mut max = first;
		let mut sum = first;

		for value in arr.iter().skip(1) {
			min = min.min(*value);
			max = max.max(*value);
			sum += value;
		}

		let mean = sum / arr.len() as f64;
		let median = if arr.len() % 2 == 0 {
			let mid = arr.len() / 2;
			(arr[mid - 1] + arr[mid]) / 2.0
		} else {
			arr[arr.len() / 2]
		};

		let mut std_sum = 0.0;
		for value in arr.iter() {
			std_sum += (value - mean).powi(2);
		}
		let std = (std_sum / arr.len() as f64).sqrt();

		ret.min = Some(min);
		ret.max = Some(max);
		ret.mean = Some(mean);
		ret.median = Some(median);
		// q1
		// q3
		ret.std = Some(std);
		ret.sum = Some(sum);

		ret
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
		let name = String::from("a");
		let s = Series::new(&name, &[1.0, 2.0, 3.0, 4.0, 5.0]);
		let a = Analyze::from(&s);

		let expect = Analyze {
			name: name.clone(),
			dtype: DataType::Float64,

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			std: Some(1.5811388),
			sum: Some(15.0),
		};

		assert_eq!(a, expect);

		let polars_expect = Analyze {
			name,
			dtype: DataType::Float64,

			min: Some(s.min().unwrap().unwrap()),
			max: Some(s.max().unwrap().unwrap()),
			mean: Some(s.mean().unwrap()),
			median: Some(s.median().unwrap()),
			std: Some(s.std(0).unwrap()),
			sum: Some(
				s.sum_reduce()
					.unwrap()
					.value()
					.try_extract::<f64>()
					.unwrap(),
			),
		};

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analyze_negative() {
		let name = String::from("a");
		let s = Series::new(&name, &[-42.0, -5.0, 0.0, 1.0, 2.0, 1001.0]);
		let a = Analyze::from(&s);

		let expect = Analyze {
			name,
			dtype: DataType::Float64,

			min: Some(-42.0),
			max: Some(1001.0),
			mean: Some(159.5),
			median: Some(0.5),
			std: Some(412.59023),
			sum: Some(957.0),
		};

		assert_eq!(a, expect);
	}

	#[test]
	fn test_analyze_option() {
		let name = String::from("a");
		let s = Series::new(&name, &[Some(1.0), None, Some(3.0), None, Some(5.0)]);
		let a = Analyze::from(&s);

		let expect = Analyze {
			name,
			dtype: DataType::Float64,

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			std: Some(1.5811388),
			sum: Some(9.0),
		};

		assert_eq!(a, expect);
	}

	#[test]
	fn test_analyze_empty() {
		let name = String::from("a");
		let empty: [f64; 0] = [];
		let s = Series::new(&name, &empty);
		let a = Analyze::from(&s);

		let expect = Analyze {
			name,
			dtype: DataType::Float64,

			min: None,
			max: None,
			mean: None,
			median: None,
			std: None,
			sum: None,
		};

		assert_eq!(a, expect);
	}

	#[test]
	fn test_analyze_str() {
		let name = String::from("a");
		let s = Series::new(&name, &["a", "b", "c"]);
		let a = Analyze::from(&s);

		let expect = Analyze {
			name,
			dtype: DataType::String,

			min: None,
			max: None,
			mean: None,
			median: None,
			std: None,
			sum: None,
		};

		assert_eq!(a, expect);
	}
}
