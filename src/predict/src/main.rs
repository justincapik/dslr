mod predict;

use std::path::PathBuf;

use clap::Parser;
use model::Model;

#[derive(Parser)]
#[command(about)]
pub struct Args {
	/// path to the csv file that will be filled with the predictions
	#[clap(default_value = "datasets/test.csv")]
	path: PathBuf,

	/// path to read the model from
	#[clap(long, short, default_value = "model.csv")]
	model: PathBuf,

	/// path to the csv file that will be filled with the predictions
	#[clap(long, short, default_value = "houses.csv")]
	output: PathBuf,
}

fn main() -> hmerr::Result<()> {
	let args = Args::parse();

	let df = load::load(&args.path)?;

	let model = Model::read(&args.model)?;

	predict::predict(&args, df, &model)?;

	Ok(())
}
