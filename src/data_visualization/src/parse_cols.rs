#![allow(dead_code)]
#![allow(unused)]

use std::{error::Error, fs::File, io::Read};

use polars::{docs::lazy, prelude::*};

use chrono::{DateTime, NaiveDate, Utc};

pub fn load_as_cols(filename: &str) -> PolarsResult<DataFrame> {
	let file = File::open(filename)?;

	let df = CsvReadOptions::default()
		.with_has_header(true)
		.try_into_reader_with_file_path(Some(filename.into()))?
		.finish()?;

	// convert and add date columns

	let year_since = "1980-01-01";
	let year_col = Series::new(
		"Year of Birth",
		df.column("Birthday").cloned()?, // .as_datetime()?
		                                 // .datetime()
		                                 // .unwrap()
		                                 // .as_datetime_iter()
		                                 // .map(|d| {
		                                 // 	d.unwrap()
		                                 // 		.date()
		                                 // 		.years_since(NaiveDate::parse_from_str(year_since, "%Y-%m-%d").unwrap())
		                                 // 		.unwrap()
		                                 // })
		                                 // .collect::<Vec<_>>(),
	);
	println!("{:?}", year_col);

	Ok(df)
}
