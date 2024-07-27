#![allow(dead_code)]
#![allow(unused)]

use std::{error::Error, process};

use polars::frame::DataFrame;
use polars::prelude::*;

mod hist_test;
mod parse_cols;
mod pp_test;
mod scatter;

fn main() -> Result<(), Box<dyn Error>> {
	let filename = "./ressources/dataset_train.csv";
	let Ok(data) = parse_cols::load_as_cols(filename) else {
		return Err("Invalid file".into());
	};

	println!("test: {:}", data);
	// println!(
	// 	"test: {:}",
	// 	data.clone()
	// 		.lazy()
	// 		.select([col("Hogwarts House"), col("Flying")])
	// 		.collect()?
	// );

	scatter::simple_scatter_plot(data)?;
	// hist_test::histogram_plot(data.clone());
	// pp_test::pair_plot(data);

	Ok(())
}
