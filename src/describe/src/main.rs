use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(about)]
struct Args {
	/// The path to the csv file to describe
	#[clap(default_value = "datasets/train.csv")]
	path: PathBuf,
}

fn main() {
	let Args { path } = Args::parse();

	dbg!(load::load(&path));
}
