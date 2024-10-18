use polars::prelude::*;

use rand_distr::num_traits::Num;

const COL_NAME: &str = "Birthday";

pub fn date(df: &mut DataFrame) -> PolarsResult<()> {
	let birth_year: Series = df
		.column(COL_NAME)?
		.clone()
		.str()?
		.into_iter()
		.map(
			|dt| /*rng.gen_range(-0.2..0.2) +*/ f64::from_str_radix(&(dt.unwrap()[0..4]), 10).unwrap(),
		)
		.collect();

	let birth_month: Series = df
		.column(COL_NAME)?
		.clone()
		.str()?
		.into_iter()
		.map(
			|dt| /*rng.gen_range(-0.2..0.2) +*/ f64::from_str_radix(&(dt.unwrap()[5..7]), 10).unwrap(),
		)
		.collect();

	let birth_day: Series = df
		.column(COL_NAME)?
		.clone()
		.str()?
		.into_iter()
		.map(|dt| f64::from_str_radix(&(dt.unwrap()[8..10]), 10).unwrap())
		.collect();

	let day_of_year = (birth_day.clone() + birth_month.clone() * 30.5).unwrap();

	df.with_column(birth_year.with_name("Year of birth".into()))?;
	df.with_column(birth_month.with_name("Month of birth".into()))?;
	// df.with_column(birth_day.with_name("Day of month of Birth"))?;
	df.with_column(day_of_year.with_name("Day of year (Birthday)".into()))?;

	Ok(())
}
