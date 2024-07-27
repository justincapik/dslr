mod group;

use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use polars::{error::PolarsResult, frame::DataFrame};

#[derive(Parser)]
#[command(about)]
pub struct Args {
	/// path to the csv file to train from
	#[clap(default_value = "datasets/train.csv")]
	path: PathBuf,

	/// path to write the model to
	#[clap(long, short, default_value = "model.csv")]
	output: PathBuf,
}

fn main() -> PolarsResult<()> {
	let args = Args::parse();

	dbg!(&args.path, &args.output);

	let df = load::load(&args.path)?;

	let grouped_row = group::row_by_label(df);

	dbg!(grouped_row
		.iter()
		.map(|(label, group)| (label, group.len()))
		.collect::<Vec<_>>());

	Ok(())
}
