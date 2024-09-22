use std::error::Error;

mod histogram;
mod pair_plot;
mod parse_cols;
mod scatter_plot;

use clap::{arg, command, value_parser};

fn main() -> Result<(), Box<dyn Error>> {
	let args = command!() // requires `cargo` feature
		.arg(
			arg!(--csv <csvname>)
				.help("Houses csv path")
				.value_parser(value_parser!(String))
				.default_value("./ressources/dataset_train.csv")
				.required(false),
		)
		.arg(
			arg!(--scatter <scatter_path>)
				.help("Scatter plot name")
				.value_parser(value_parser!(String))
				.default_value("Scatter_plot.png")
				.required(false),
		)
		.arg(
			arg!(--hist <hist_path>)
				.help("Histogram plot name")
				.value_parser(value_parser!(String))
				.default_value("Histogram.png")
				.required(false),
		)
		.arg(
			arg!(--pair <pair_path>)
				.help("Pair Plot plot name")
				.value_parser(value_parser!(String))
				.default_value("Pair_Plot.png")
				.required(false),
		)
		.get_matches();

	let filename = args
		.get_one::<String>("csv")
		.expect("default ensures there is always a value");
	let Ok(mut data) = parse_cols::load_as_cols(filename) else {
		return Err("Invalid file".into());
	};

	// TODO: polars_core::frame::DataFrame and
	// polare::prelude::DataFrame clash, but both seem to be the same
	// Alex sos
	// the rest works though
	// let mut data = load::load(filename)?;

	parse_cols::transform_data(&mut data)?;

	println!("Creating scatter plot...");
	let name = args
		.get_one::<String>("scatter")
		.expect("default garuntees a value");
	scatter_plot::simple_scatter_plot(&data, name)?;

	println!("Creating histogram...");
	let name = args
		.get_one::<String>("hist")
		.expect("default garuntees a value");
	histogram::histogram_plot(&data, name)?;

	println!("Creating pair plot...");
	let name = args
		.get_one::<String>("pair")
		.expect("default garuntees a value");
	pair_plot::pair_plot(&data, name)?;

	Ok(())
}
