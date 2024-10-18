/// since the error is never critical, thus for optimization:
/// feature::parse() is not returning a descriptive error
pub fn parse(series: Series) -> Result<Vec<f64>, ()> {
	let Ok(feature) = series.f64() else {
		return Err(());
	};

	let col: Vec<f64> = feature
		.into_iter()
		.flatten()
		.filter(|x| *x != 0.0)
		.collect::<Vec<f64>>();

	if col.is_empty() {
		return Err(());
	}

	Ok(col)
}
