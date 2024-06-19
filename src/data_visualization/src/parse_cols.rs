#![allow(dead_code)]
#![allow(unused)]

use std::{error::Error, fs::File, io::Read};

use polars::prelude::*;
	
pub fn load_as_cols(filename: &str) -> PolarsResult<DataFrame> {
	
	let file = File::open(filename)?;
	
	CsvReadOptions::default()
		.with_has_header(true)
		.try_into_reader_with_file_path(Some(filename.into()))?
		.finish()
}
