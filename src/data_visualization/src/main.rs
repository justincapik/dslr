#![allow(dead_code)]
#![allow(unused)]

use std::{error::Error, process};

use polars::frame::DataFrame;
use polars::prelude::*;

mod parse_cols;
mod plot;

fn main() -> Result<(), Box<dyn Error>> {
	let filename = "./ressources/dataset_train.csv";
	let Ok(data) = parse_cols::load_as_cols(filename) else {
		return Err("Invalid file".into());
	};

	println!("test: {:}", data);
	println!(
		"test: {:}",
		data.clone()
			.lazy()
			.select([col("Hogwarts House"), col("Flying")])
			.collect()?
	);

	plot::simple_scatter_plot(data)?;

	plot::normalized_histogram();
	plot::test_scatter_plot();

	Ok(())
}
