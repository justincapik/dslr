use std::path::PathBuf;

use polars::prelude::*;
pub fn load(path: impl Into<PathBuf>) -> PolarsResult<()> {
	let df = CsvReadOptions::default()
		.with_infer_schema_length(None)
		.with_has_header(true)
		.try_into_reader_with_file_path(Some(path.into()))?
		.finish()?;

	dbg!(&df.select(["Flying"])?);
	println!("{}", df);

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_load() {
		dbg!(load("../../ressources/dataset_test.csv").expect("Failed to load data"));
		dbg!(load("../../ressources/dataset_train.csv").expect("Failed to load data"));
	}
}
