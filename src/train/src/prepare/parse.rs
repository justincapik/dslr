use std::collections::HashMap;

use analyze::Analysis;
use float::Float;
use polars::{frame::DataFrame, prelude::DataType};

use super::{Datasets, GroupedDatasets};

const UNKNOWN_LABEL: &str = "UNKNOWN";
const MOD_SPLIT_FACTOR: usize = 3;

pub fn datasets(df: &DataFrame, analysis: &[Analysis]) -> GroupedDatasets {
	let mut grouped_datasets = HashMap::new();

	let capacity = get_capacity(df);

	let mut iters = df.iter().map(|s| s.iter()).collect::<Vec<_>>();

	for row in 0..df.height() {
		let mut label: Option<String> = None;
		let mut row_data = Vec::with_capacity(capacity);

		for iter in &mut iters {
			let cell = iter
				.next()
				.unwrap_or_else(|| panic!("there must be a value on row {row}"));

			if cell.dtype().is_string() && label.is_none() {
				label = Some(
					cell.get_str()
						.expect("could not cast cell {cell} to string")
						.to_owned(),
				);
			}

			if cell.dtype().is_float() || cell.is_null() {
				row_data.push(
					cell.cast(&DataType::Float32)
						.try_extract::<Float>()
						.unwrap_or(
							analysis[row_data.len()]
								.mean
								.unwrap_or_else(|| panic!("float feature must have a mean")),
						),
				);
			}
		}

		let label = label.unwrap_or(String::from(UNKNOWN_LABEL));

		let datasets = grouped_datasets.entry(label).or_insert_with(|| Datasets {
			training: Vec::new(),
			testing: Vec::new(),
		});

		if datasets.training.is_empty() {
			datasets.training.push(row_data);
		} else if datasets.testing.is_empty() || row % MOD_SPLIT_FACTOR == 0 {
			datasets.testing.push(row_data);
		} else {
			datasets.training.push(row_data);
		}
	}

	grouped_datasets
}

fn get_capacity(df: &DataFrame) -> usize {
	let mut capacity = 0;

	for col in df.get_columns() {
		if col.dtype().is_float() {
			capacity += 1;
		}
	}

	capacity
}

#[cfg(test)]
mod tests {
	use super::super::features_analysis;
	use super::*;
	use polars::prelude::*;

	#[test]
	fn test_parse_basic() {
		let df = DataFrame::new(vec![
			Series::new("label", &["a", "b", "a", "c"]),
			Series::new("one", &[1.0, 2.0, 3.0, 4.0]),
			Series::new("two", &[5.0, 6.0, 7.0, 8.0]),
		])
		.unwrap();

		let grouped_datasets = datasets(&df, &features_analysis(&df));

		assert_eq!(grouped_datasets.len(), 3);

		assert_eq!(
			grouped_datasets.get("a").unwrap(),
			&Datasets {
				training: vec![vec![1.0, 5.0]],
				testing: vec![vec![3.0, 7.0]]
			}
		);
		assert_eq!(
			grouped_datasets.get("b").unwrap(),
			&Datasets {
				training: vec![vec![2.0, 6.0]],
				testing: vec![]
			}
		);
		assert_eq!(
			grouped_datasets.get("c").unwrap(),
			&Datasets {
				training: vec![vec![4.0, 8.0]],
				testing: vec![]
			}
		);
	}

	#[test]
	fn test_parse_missing_label() {
		let df = DataFrame::new(vec![
			Series::new("one", &[1.0, 2.0, 3.0, 4.0]),
			Series::new("two", &[5.0, 6.0, 7.0, 8.0]),
		])
		.unwrap();

		let grouped_datasets = datasets(&df, &features_analysis(&df));

		assert_eq!(grouped_datasets.len(), 1);

		assert_eq!(
			grouped_datasets.get(UNKNOWN_LABEL).unwrap(),
			&Datasets {
				training: vec![vec![1.0, 5.0], vec![3.0, 7.0]],
				testing: vec![vec![2.0, 6.0], vec![4.0, 8.0]]
			}
		);
	}

	#[test]
	fn test_parse_label_in_middle() {
		let df = DataFrame::new(vec![
			Series::new("one", &[1.0, 2.0, 3.0, 4.0]),
			Series::new("label", &["a", "a", "b", "c"]),
			Series::new("two", &[5.0, 6.0, 7.0, 8.0]),
		])
		.unwrap();

		let grouped_datasets = datasets(&df, &features_analysis(&df));

		assert_eq!(grouped_datasets.len(), 3);

		assert_eq!(
			grouped_datasets.get("a").unwrap(),
			&Datasets {
				training: vec![vec![1.0, 5.0]],
				testing: vec![vec![2.0, 6.0]]
			}
		);
		assert_eq!(
			grouped_datasets.get("b").unwrap(),
			&Datasets {
				training: vec![vec![3.0, 7.0]],
				testing: vec![]
			}
		);
		assert_eq!(
			grouped_datasets.get("c").unwrap(),
			&Datasets {
				training: vec![vec![4.0, 8.0]],
				testing: vec![]
			}
		);
	}

	#[test]
	fn test_parse_with_integer() {
		let df = DataFrame::new(vec![
			Series::new("label", &["a", "a", "b", "c"]),
			Series::new("one", &[1, 2, 3, 4]),
			Series::new("two", &[5.0, 6.0, 7.0, 8.0]),
		])
		.unwrap();

		let grouped_datasets = datasets(&df, &features_analysis(&df));

		assert_eq!(grouped_datasets.len(), 3);

		assert_eq!(
			grouped_datasets.get("a").unwrap(),
			&Datasets {
				training: vec![vec![5.0]],
				testing: vec![vec![6.0]],
			}
		);
		assert_eq!(
			grouped_datasets.get("b").unwrap(),
			&Datasets {
				training: vec![vec![7.0]],
				testing: vec![],
			}
		);
		assert_eq!(
			grouped_datasets.get("c").unwrap(),
			&Datasets {
				training: vec![vec![8.0]],
				testing: vec![],
			}
		);
	}
}
