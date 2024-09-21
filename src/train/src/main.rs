mod learn;
mod loss;
mod prepare;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use polars::error::PolarsResult;

use float::Float;

#[derive(ValueEnum, Default, Clone, Copy, PartialEq)]
pub enum Normalization {
	MinMax,
	#[default]
	StdDev,
}

#[derive(Parser)]
#[command(about)]
pub struct Args {
	/// path to the csv file to train from
	#[clap(default_value = "datasets/train.csv")]
	path: PathBuf,

	/// path to write the model to
	#[clap(long, short, default_value = "model.csv")]
	output: PathBuf,

	/// learning rate
	#[clap(long = "rate", short = 'r', default_value = "0.01")]
	learning_rate: Float,

	/// number of gradient descent iterations
	#[clap(long = "iter", short = 'i', default_value = "1000")]
	iteration: usize,

	/// data normalization method
	#[clap(long = "norm", short = 'n', default_value = "std-dev")]
	normalization: Normalization,
}

fn main() -> PolarsResult<()> {
	let args = Args::parse();

	let df = load::load(&args.path)?;

	let grouped_datasets = prepare::prepare(&args, df);

	let model = learn::learn(&args, &grouped_datasets);

	loss::print_result(&grouped_datasets, &model);

	Ok(())
}
