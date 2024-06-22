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
	q1: Option<f64>,
	q3: Option<f64>,
	std: Option<f64>,
	sum: Option<f64>,
}

type TableRecord<'s> = [&'s str; 10];

pub fn compute(df: DataFrame, args: &Args) -> PolarsResult<(Table, Vec<DataType>)> {
	let mut types = Vec::with_capacity(df.width());

	let mut builder = Builder::default();

	builder.push_record(Analyze::HEADERS);

	for series in df.get_columns() {
		let analyze = Analyze::from(series);

		let name = truncate(&analyze.name, 10);

		builder.push_record([
			&name,
			&analyze.dtype.to_string(),
			&to_string(analyze.min, args),
			&to_string(analyze.max, args),
			&to_string(analyze.mean, args),
			&to_string(analyze.median, args),
			&to_string(analyze.q1, args),
			&to_string(analyze.q3, args),
			&to_string(analyze.std, args),
			&to_string(analyze.sum, args),
		]);

		types.push(analyze.dtype);
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
			q1: None,
			q3: None,
			std: None,
			sum: None,
		};

		if !ret.dtype.is_numeric() {
			return ret;
		}

		let mut arr = series
			.cast(&DataType::Float64)
			.expect("could not cast series to f64")
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
		let (q1, q3) = if arr.len() % 4 == 0 {
			let mid = arr.len() / 4;
			(
				(arr[mid - 1] + arr[mid]) / 2.0,
				(arr[mid * 3 - 1] + arr[mid * 3]) / 2.0,
			)
		} else {
			(arr[arr.len() / 4], arr[arr.len() * 3 / 4])
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
		ret.q1 = Some(q1);
		ret.q3 = Some(q3);
		ret.std = Some(std);
		ret.sum = Some(sum);

		ret
	}
}

impl Analyze {
	const HEADERS: TableRecord<'static> = [
		"column", "T", "min", "max", "mean", "median", "q1", "q3", "std", "sum",
	];
}

fn truncate(s: &str, len: usize) -> String {
	if s.len() <= len {
		return s.to_owned();
	}

	format!("{}…", &s[..len - 1])
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

	fn polars_expect(name: String, dtype: DataType, series: Series) -> Analyze {
		Analyze {
			name,
			dtype,

			min: series.min().unwrap(),
			max: series.max().unwrap(),
			mean: series.mean(),
			median: series.median(),
			q1: match series.quantile_reduce(0.25, QuantileInterpolOptions::Linear) {
				Ok(x) => x.value().try_extract::<f64>().ok(),
				_ => None,
			},
			q3: match series.quantile_reduce(0.25, QuantileInterpolOptions::Linear) {
				Ok(x) => x.value().try_extract::<f64>().ok(),
				_ => None,
			},
			std: series.std(0),
			sum: match series.sum_reduce() {
				Ok(x) => x
					.value()
					.try_extract::<f64>()
					.ok()
					.map(|x| if x == 0.0 { None } else { Some(x) })
					.flatten(),
				_ => None,
			},
		}
	}

	#[test]
	fn test_analyze_basic() {
		let name = String::from("a");
		let dtype = DataType::Float64;
		let s = Series::new(&name, &[1.0, 2.0, 3.0, 4.0, 5.0]);

		let a = Analyze::from(&s);

		let expect = Analyze {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			q1: Some(2.0),
			q3: Some(4.0),
			std: Some(1.4142135623730951),
			sum: Some(15.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analyze_unsorted() {
		let name = String::from("a");
		let dtype = DataType::Float64;
		let s = Series::new(&name, &[5.0, 2.0, 4.0, 1.0, 3.0]);

		let a = Analyze::from(&s);

		let expect = Analyze {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			q1: Some(2.0),
			q3: Some(4.0),
			std: Some(1.4142135623730951),
			sum: Some(15.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analyze_negative() {
		let name = String::from("a");
		let dtype = DataType::Float64;
		let s = Series::new(&name, &[-42.0, -5.0, 0.0, 1.0, 2.0, 1001.0]);

		let a = Analyze::from(&s);

		let expect = Analyze {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(-42.0),
			max: Some(1001.0),
			mean: Some(159.5),
			median: Some(0.5),
			q1: Some(-2.5),
			q3: Some(1.5),
			std: Some(376.64162896135986),
			sum: Some(957.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analyze_option() {
		let name = String::from("a");
		let dtype = DataType::Float64;
		let s = Series::new(&name, &[Some(1.0), None, Some(3.0), None, Some(5.0)]);

		let a = Analyze::from(&s);

		let expect = Analyze {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			q1: Some(2.0),
			q3: Some(4.0),
			std: Some(1.632993161855452),
			sum: Some(9.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analyze_int() {
		let name = String::from("a");
		let dtype = DataType::Int32;
		let s = Series::new(&name, &[1, 2, 3, 4, 5]);

		let a = Analyze::from(&s);

		let expect = Analyze {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			q1: Some(2.0),
			q3: Some(4.0),
			std: Some(1.4142135623730951),
			sum: Some(15.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analyze_empty() {
		let name = String::from("a");
		let dtype = DataType::Float64;
		let empty: [f64; 0] = [];
		let s = Series::new(&name, &empty);

		let a = Analyze::from(&s);

		let expect = Analyze {
			name: name.clone(),
			dtype: dtype.clone(),

			min: None,
			max: None,
			mean: None,
			median: None,
			q1: None,
			q3: None,
			std: None,
			sum: None,
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analyze_str() {
		let name = String::from("a");
		let dtype = DataType::String;
		let s = Series::new(&name, &["a", "b", "c"]);

		let a = Analyze::from(&s);

		let expect = Analyze {
			name: name.clone(),
			dtype: dtype.clone(),

			min: None,
			max: None,
			mean: None,
			median: None,
			q1: None,
			q3: None,
			std: None,
			sum: None,
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}
}
