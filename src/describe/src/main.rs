mod describe;

use std::path::PathBuf;

use clap::Parser;
use polars::error::PolarsResult;

#[derive(Parser)]
#[command(about)]
pub struct Args {
	/// path to the csv file to describe
	#[clap(default_value = "datasets/train.csv")]
	path: PathBuf,

	/// round the output to the given number of decimal places
	#[clap(long, short, default_value = "2")]
	round: u8,
	/*
	/// rotate the output 90 degrees
	#[clap(short, long)]
	rotate: bool,
	*/
}

fn main() -> PolarsResult<()> {
	let args = Args::parse();

	let df = load::load(&args.path)?;

	describe::describe(df, &args)
}
