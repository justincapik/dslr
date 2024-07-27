use std::collections::HashMap;

use polars::prelude::*;

const UNKNOWN_LABEL: &str = "UNKNOWN";

pub fn row_by_label(df: DataFrame) -> HashMap<String, Vec<Vec<f32>>> {
	let mut grouped_row = HashMap::new();

	let capacity = get_capacity(&df);

	let mut iters = df.iter().map(|s| s.iter()).collect::<Vec<_>>();

	for row in 0..df.height() {
		let mut label: Option<String> = None;
		let mut row_data = Vec::with_capacity(capacity);

		for iter in &mut iters {
			let cell = iter
				.next()
				.expect(&format!("there must be a value on row {row}"));

			if cell.dtype().is_string() && label.is_none() {
				label = Some(
					cell.get_str()
						.expect("could not cast cell {cell} to string")
						.to_owned(),
				);
			}

			if cell.dtype().is_float() {
				row_data.push(
					cell.cast(&DataType::Float32)
						.try_extract::<f32>()
						.unwrap_or(0.0),
				);
			}
		}

		let label = label.unwrap_or(String::from(UNKNOWN_LABEL));

		let entry = grouped_row.entry(label).or_insert_with(Vec::new);

		entry.push(row_data);
	}

	grouped_row
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
	use super::*;

	#[test]
	fn test_row_by_label_basic() {
		let df = DataFrame::new(vec![
			Series::new("label", &["a", "a", "b", "c"]),
			Series::new("one", &[1.0, 2.0, 3.0, 4.0]),
			Series::new("two", &[5.0, 6.0, 7.0, 8.0]),
		])
		.unwrap();

		let grouped_row = row_by_label(df);

		assert_eq!(grouped_row.len(), 3);

		assert_eq!(
			grouped_row.get("a").unwrap(),
			&vec![vec![1.0, 5.0], vec![2.0, 6.0]]
		);
		assert_eq!(grouped_row.get("b").unwrap(), &vec![vec![3.0, 7.0]]);
		assert_eq!(grouped_row.get("c").unwrap(), &vec![vec![4.0, 8.0]]);
	}

	#[test]
	fn test_row_by_label_missing_label() {
		let df = DataFrame::new(vec![
			Series::new("one", &[1.0, 2.0, 3.0, 4.0]),
			Series::new("two", &[5.0, 6.0, 7.0, 8.0]),
		])
		.unwrap();

		let grouped_row = row_by_label(df);

		assert_eq!(grouped_row.len(), 1);

		assert_eq!(
			grouped_row.get(UNKNOWN_LABEL).unwrap(),
			&vec![
				vec![1.0, 5.0],
				vec![2.0, 6.0],
				vec![3.0, 7.0],
				vec![4.0, 8.0]
			]
		);
	}

	#[test]
	fn test_row_by_label_label_in_middle() {
		let df = DataFrame::new(vec![
			Series::new("one", &[1.0, 2.0, 3.0, 4.0]),
			Series::new("label", &["a", "a", "b", "c"]),
			Series::new("two", &[5.0, 6.0, 7.0, 8.0]),
		])
		.unwrap();

		let grouped_row = row_by_label(df);

		assert_eq!(grouped_row.len(), 3);

		assert_eq!(
			grouped_row.get("a").unwrap(),
			&vec![vec![1.0, 5.0], vec![2.0, 6.0]]
		);
		assert_eq!(grouped_row.get("b").unwrap(), &vec![vec![3.0, 7.0]]);
		assert_eq!(grouped_row.get("c").unwrap(), &vec![vec![4.0, 8.0]]);
	}

	#[test]
	fn test_row_by_label_with_integer() {
		let df = DataFrame::new(vec![
			Series::new("label", &["a", "a", "b", "c"]),
			Series::new("one", &[1, 2, 3, 4]),
			Series::new("two", &[5.0, 6.0, 7.0, 8.0]),
		])
		.unwrap();

		let grouped_row = row_by_label(df);

		assert_eq!(grouped_row.len(), 3);

		assert_eq!(grouped_row.get("a").unwrap(), &vec![vec![5.0], vec![6.0]]);
		assert_eq!(grouped_row.get("b").unwrap(), &vec![vec![7.0]]);
		assert_eq!(grouped_row.get("c").unwrap(), &vec![vec![8.0]]);
	}
}
