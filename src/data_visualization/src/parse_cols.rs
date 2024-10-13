use polars::prelude::*;

use rand_distr::num_traits::Num;

// use rand;

pub fn load_as_cols(filename: &str) -> PolarsResult<DataFrame> {
	let df = CsvReadOptions::default()
		.with_has_header(true)
		.try_into_reader_with_file_path(Some(filename.into()))?
		.finish()?;

	Ok(df)
}

pub fn transform_data(df: &mut DataFrame) -> PolarsResult<()> {
	// convert and add date columns

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

	df.with_column(birth_year.clone().with_name("Year of birth".into()))?;
	df.with_column(birth_month.clone().with_name("Month of birth".into()))?;
	// df.with_column(birth_day.clone().with_name("Day of month of Birth"))?;
	df.with_column(
		day_of_year
			.clone()
			.with_name("Day of year (Birthday)".into()),
	)?;

	Ok(())
}
