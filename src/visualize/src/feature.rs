use polars::series::Series;

pub fn parse(series: &Series) -> Option<Vec<f64>> {
	let Ok(feature) = series.f64() else {
		return None;
	};

	let col: Vec<f64> = feature
		.into_iter()
		.flatten()
		.filter(|x| *x != 0.0)
		.collect::<Vec<f64>>();

	if col.is_empty() {
		return None;
	}

	Some(col)
}
