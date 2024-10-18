use std::path::PathBuf;

use clap::{arg, command, value_parser};

pub struct Args {
	pub csv: PathBuf,
	pub output: PathBuf,
}

pub fn parse(default_output: &'static str) -> Args {
	let args = command!()
		.arg(
			arg!(--csv <path>)
				.help("dataset csv path")
				.value_parser(value_parser!(PathBuf))
				.default_value("./datasets/train.csv")
				.required(false),
		)
		.arg(
			arg!(--output <path>)
				.help("output path of the graph")
				.value_parser(value_parser!(PathBuf))
				.default_value(default_output)
				.required(false),
		)
		.get_matches();

	Args {
		csv: args
			.get_one::<PathBuf>("csv")
			.expect("default ensures there is always a value")
			.to_owned(),
		output: args
			.get_one::<PathBuf>("output")
			.expect("default ensures there is always a value")
			.to_owned(),
	}
}
