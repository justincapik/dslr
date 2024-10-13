use polars::prelude::{DataType, Series};

use float::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Analysis {
	pub name: String,
	pub dtype: DataType,

	pub min: Option<Float>,
	pub max: Option<Float>,
	pub mean: Option<Float>,
	pub median: Option<Float>,
	pub q1: Option<Float>,
	pub q3: Option<Float>,
	pub std: Option<Float>,
	pub sum: Option<Float>,
}

impl From<Series> for Analysis {
	fn from(series: Series) -> Self {
		Analysis::from(&series)
	}
}

impl From<&Series> for Analysis {
	fn from(series: &Series) -> Self {
		let name = series.name().to_string();
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
			.cast(&DataType::Float32)
			.expect("could not cast series to f32")
			.f32()
			.expect("could not extract series as f32 iterator")
			.into_iter()
			.flatten()
			.collect::<Vec<Float>>();

		arr.sort_by(|a, b| a.total_cmp(b));

		let Some(min) = arr.first().copied() else {
			return ret;
		};
		let max = arr.last().copied().unwrap();

		let sum: Float = arr.iter().sum();

		let mean = sum / arr.len() as Float;
		let median = if arr.len() % 2 == 0 {
			let mid = arr.len() / 2;
			(arr[mid - 1] + arr[mid]) / 2.0
		} else {
			arr[arr.len() / 2]
		};
		let (q1, q3) = if arr.len() % 4 == 0 {
			let quarter = arr.len() / 4;
			(
				(arr[quarter - 1] + arr[quarter]) / 2.0,
				(arr[quarter * 3 - 1] + arr[quarter * 3]) / 2.0,
			)
		} else {
			(arr[arr.len() / 4], arr[arr.len() * 3 / 4])
		};

		let mut std_sum = 0.0;
		for value in arr.iter() {
			std_sum += (value - mean).powi(2);
		}
		let std = (std_sum / arr.len() as Float).sqrt();

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

#[cfg(test)]
mod tests {
	use super::*;
	use polars::prelude::*;

	fn polars_expect(name: String, dtype: DataType, series: Series) -> Analysis {
		Analysis {
			name,
			dtype,

			min: series.min().unwrap(),
			max: series.max().unwrap(),
			mean: series.mean().map(|x| x as Float),
			median: series.median().map(|x| x as Float),
			q1: match series.quantile_reduce(0.25, QuantileInterpolOptions::Lower) {
				Ok(x) => x.value().try_extract::<Float>().ok(),
				_ => None,
			},
			q3: match series.quantile_reduce(0.75, QuantileInterpolOptions::Higher) {
				Ok(x) => x.value().try_extract::<Float>().ok(),
				_ => None,
			},
			std: series.std(0).map(|x| x as Float),
			sum: match series.sum_reduce() {
				Ok(x) => x.value().try_extract::<Float>().ok().and_then(|x| {
					if x == 0.0 {
						None
					} else {
						Some(x)
					}
				}),
				_ => None,
			},
		}
	}

	#[test]
	fn test_analysis_basic() {
		let name = String::from("a");
		let dtype = DataType::Float64;
		let s = Series::new((&name).into(), &[1.0, 2.0, 3.0, 4.0, 5.0]);

		let a = Analysis::from(&s);

		let expect = Analysis {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			q1: Some(2.0),
			q3: Some(4.0),
			std: Some(std::f32::consts::SQRT_2),
			sum: Some(15.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analysis_unsorted() {
		let name = String::from("a");
		let dtype = DataType::Float64;
		let s = Series::new((&name).into(), &[5.0, 2.0, 4.0, 1.0, 3.0]);

		let a = Analysis::from(&s);

		let expect = Analysis {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			q1: Some(2.0),
			q3: Some(4.0),
			std: Some(std::f32::consts::SQRT_2),
			sum: Some(15.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analysis_negative() {
		let name = String::from("a");
		let dtype = DataType::Float64;
		let s = Series::new((&name).into(), &[-42.0, -5.0, 0.0, 1.0, 2.0, 1001.0]);

		let a = Analysis::from(&s);

		let expect = Analysis {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(-42.0),
			max: Some(1001.0),
			mean: Some(159.5),
			median: Some(0.5),
			q1: Some(-5.0),
			q3: Some(2.0),
			std: Some(376.64162896135986),
			sum: Some(957.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analysis_option() {
		let name = String::from("a");
		let dtype = DataType::Float64;
		let s = Series::new(
			(&name).into(),
			&[Some(1.0), None, Some(3.0), None, Some(5.0)],
		);

		let a = Analysis::from(&s);

		let expect = Analysis {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			q1: Some(1.0),
			q3: Some(5.0),
			std: Some(1.632993161855452),
			sum: Some(9.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analysis_int() {
		let name = String::from("a");
		let dtype = DataType::Int32;
		let s = Series::new((&name).into(), &[1, 2, 3, 4, 5]);

		let a = Analysis::from(&s);

		let expect = Analysis {
			name: name.clone(),
			dtype: dtype.clone(),

			min: Some(1.0),
			max: Some(5.0),
			mean: Some(3.0),
			median: Some(3.0),
			q1: Some(2.0),
			q3: Some(4.0),
			std: Some(std::f32::consts::SQRT_2),
			sum: Some(15.0),
		};

		assert_eq!(a, expect);

		let polars_expect = polars_expect(name, dtype, s);

		assert_eq!(a, polars_expect);
	}

	#[test]
	fn test_analysis_empty() {
		let name = String::from("a");
		let dtype = DataType::Float32;
		let empty: [Float; 0] = [];
		let s = Series::new((&name).into(), &empty);

		let a = Analysis::from(&s);

		let expect = Analysis {
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
	fn test_analysis_str() {
		let name = String::from("a");
		let dtype = DataType::String;
		let s = Series::new((&name).into(), &["a", "b", "c"]);

		let a = Analysis::from(&s);

		let expect = Analysis {
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
