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
	
	
	
	
	// let file = File::open(filename)?;
	// let mut rdr = csv::Reader::from_reader(file);

	// let mut _line_iterator = rdr.records();


	// for result in rdr.records() {
	//     // The iterator yields Result<StringRecord, Error>, so we check the
	//     // error here..
	//     let _record = result;
	//     //println!("{:?}", record);
	// }

	// dbg!(rdr.headers());
	// Err(Box::new(std::fmt::Error))
}
