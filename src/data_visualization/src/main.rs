#![allow(dead_code)]
#![allow(unused)]

use std::{error::Error, process};

use polars::frame::DataFrame;
use polars::prelude::*;

mod histogram;
mod pair_plot;
mod parse_cols;
mod scatter_plot;

fn main() -> Result<(), Box<dyn Error>> {
	let filename = "./ressources/dataset_train.csv";
	let Ok(data) = parse_cols::load_as_cols(filename) else {
		return Err("Invalid file".into());
	};

	let data = parse_cols::transform_data(data)?;

	println!("test: {:}", data);

	scatter_plot::simple_scatter_plot(data.clone())?;
	histogram::histogram_plot(data.clone())?;
	// pair_plot::pair_plot(data);

	Ok(())
}
