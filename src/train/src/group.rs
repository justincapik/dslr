use std::collections::HashMap;

use polars::prelude::*;

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

		let label = label.unwrap_or(String::from("UNKNOWN"));

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
