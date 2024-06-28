use std::path::PathBuf;

use polars::prelude::*;

pub fn load(path: impl Into<PathBuf>) -> PolarsResult<DataFrame> {
	let df = CsvReadOptions::default()
		.with_infer_schema_length(None)
		.with_has_header(true)
		.try_into_reader_with_file_path(Some(path.into()))?
		.finish()?;

	Ok(df)
}
