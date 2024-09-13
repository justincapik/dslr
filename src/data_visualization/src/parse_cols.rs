#![allow(dead_code)]
#![allow(unused)]

use std::{error::Error, fs::File, io::Read};

use polars::{docs::lazy, prelude::*};

use chrono::{DateTime, NaiveDate, Utc};

use rand_distr::num_traits::Num;
use std::str::FromStr;

use rand;
use rand::Rng;
// use rand::Rng::gen_range;

pub fn load_as_cols(filename: &str) -> PolarsResult<DataFrame> {
	let file = File::open(filename)?;

	let df = CsvReadOptions::default()
		.with_has_header(true)
		.try_into_reader_with_file_path(Some(filename.into()))?
		.finish()?;

	Ok(df)
}

pub fn transform_data(df: DataFrame) -> PolarsResult<DataFrame> {
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

	// let out = df
	// 	.clone()
	// 	.lazy()
	// 	.select([col("Birthday")
	// 		.str()
	// 		.to_datetime(
	// 			Some(TimeUnit::Microseconds),
	// 			None,
	// 			StrptimeOptions::default(),
	// 			lit("raise"),
	// 		)
	// 		.alias("day")])
	// 	.collect()?;
	// println!("{}", out);

	let mut rng = rand::thread_rng();

	let birth_year: Series = df
		.column("Birthday")?
		.clone()
		.str()?
		.into_iter()
		.map(
			|dt| /*rng.gen_range(-0.2..0.2) +*/ f64::from_str_radix(&(dt.unwrap()[0..4]), 10).unwrap(),
		)
		.collect();

	let birth_month: Series = df
		.column("Birthday")?
		.clone()
		.str()?
		.into_iter()
		.map(
			|dt| /*rng.gen_range(-0.2..0.2) +*/ f64::from_str_radix(&(dt.unwrap()[5..7]), 10).unwrap(),
		)
		.collect();

	let birth_day: Series = df
		.column("Birthday")?
		.clone()
		.str()?
		.into_iter()
		.map(|dt| f64::from_str_radix(&(dt.unwrap()[8..10]), 10).unwrap())
		.collect();

	let day_of_year = (birth_day.clone() + birth_month.clone() * 30.5).unwrap();

	let mut binding = df.clone();
	binding.with_column(birth_year.clone().with_name("Year of birth"));
	binding.with_column(birth_month.clone().with_name("Month of birth"));
	// binding.with_column(birth_day.clone().with_name("Day of month of Birth"));
	binding.with_column(day_of_year.clone().with_name("Day of year (Birthday)"));

	let new = binding.clone();

	println!("{:?}", birth_month);

	let df = new.clone();

	Ok(df)
}
