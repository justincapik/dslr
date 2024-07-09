use std::path::PathBuf;

use clap::Parser;
use polars::error::PolarsResult;

#[derive(Parser)]
#[command(about)]
pub struct Args {
	/// path to the csv file to train from
	#[clap(default_value = "datasets/train.csv")]
	path: PathBuf,

	/// path to write the model to
	#[clap(long, short, default_value = "model.csv")]
	ouput: PathBuf,
}

fn main() -> PolarsResult<()> {
	let args = Args::parse();

	dbg!(&args.path, &args.ouput);

	Ok(())
}
