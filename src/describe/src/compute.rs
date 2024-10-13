use float::Float;
use polars::prelude::*;
use tabled::{builder::Builder, Table};

use analyze::Analysis;

use crate::Args;

type TableRecord<'s> = [&'s str; 9];

const HEADERS: TableRecord<'static> = [
	"column", "T", "min", "max", "mean", "median", "q1", "q3", "std",
];

pub fn compute(df: DataFrame, args: &Args) -> PolarsResult<(Table, Vec<DataType>)> {
	let mut types = Vec::with_capacity(df.width());

	let mut builder = Builder::default();

	builder.push_record(HEADERS);

	for series in df.get_columns() {
		if !args.full && !series.dtype().is_numeric() {
			continue;
		}

		let analysis = Analysis::from(series);

		let name = truncate(&analysis.name, 10);

		let record: TableRecord = [
			&name,
			&analysis.dtype.to_string(),
			&to_string(analysis.min, args),
			&to_string(analysis.max, args),
			&to_string(analysis.mean, args),
			&to_string(analysis.median, args),
			&to_string(analysis.q1, args),
			&to_string(analysis.q3, args),
			&to_string(analysis.std, args),
		];
		builder.push_record(record);

		types.push(analysis.dtype);
	}

	Ok((builder.build(), types))
}

fn truncate(s: &str, len: usize) -> String {
	if s.len() <= len {
		return s.to_owned();
	}

	format!("{}…", &s[..len - 1])
}

fn to_string(n: Option<Float>, args: &Args) -> String {
	let Some(n) = n else {
		return String::new();
	};

	format!("{:.1$}", n, args.round as usize)
}
