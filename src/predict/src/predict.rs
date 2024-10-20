use hypothesis::one_vs_all;
use polars::{prelude::*, series::SeriesIter};

use float::Float;
use model::Model;

use crate::Args;

const INDEX_COLUMN: &str = "Index";

pub fn predict(args: &Args, df: DataFrame, model: &Model) -> hmerr::Result<()> {
	let mut wtr = csv::Writer::from_path(&args.output)?;

	wtr.write_record([INDEX_COLUMN, &model.label_name])?;

	let capacity = get_capacity(&df);

	let mut iters = df.iter().map(|s| s.iter()).collect::<Vec<_>>();

	for idx in 0..df.height() {
		let row = construct_row(idx, &mut iters, capacity, model);

		let label = one_vs_all(&row, model);

		wtr.write_record([&idx.to_string(), &label])?;
	}

	wtr.flush()?;
	Ok(())
}

fn construct_row(
	idx: usize,
	row: &mut Vec<SeriesIter<'_>>,
	capacity: usize,
	model: &Model,
) -> Vec<Float> {
	let mut row_data = Vec::with_capacity(capacity);

	let mut skipped_first_null = false;

	for iter in row {
		let cell = iter
			.next()
			.unwrap_or_else(|| panic!("there must be a value on row {idx}"));

		if cell.is_null() && !skipped_first_null {
			skipped_first_null = true;
			continue;
		}

		if cell.dtype().is_float() || cell.is_null() {
			let feature_idx = row_data.len();

			let x = cell
				.try_extract::<Float>()
				.unwrap_or(model.means[feature_idx]);

			let (a, b) = model.normalization_factors[feature_idx];
			let normalized = (x - a) / b;

			row_data.push(normalized);
		}
	}

	row_data
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
