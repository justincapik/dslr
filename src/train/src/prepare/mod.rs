mod normalize;
mod parse;

use std::collections::HashMap;

use analyze::Analysis;
use float::Float;
use polars::prelude::*;

use crate::Args;

pub type GroupedDatasets = HashMap<String, Datasets>;

#[derive(Debug, PartialEq)]
pub struct Datasets {
	pub training: Dataset,
	pub testing: Dataset,
}
pub type Dataset = Vec<Features>;
pub type Features = Vec<Float>;

pub fn prepare(args: &Args, df: DataFrame) -> GroupedDatasets {
	let analysis = features_analysis(&df);

	let mut grouped_datasets = parse::datasets(&df, &analysis);

	normalize::normalize(args, &mut grouped_datasets, &analysis);

	grouped_datasets
}

fn features_analysis(df: &DataFrame) -> Vec<Analysis> {
	let mut features_analysis = Vec::new();

	for col in df.get_columns() {
		if col.dtype().is_float() {
			features_analysis.push(Analysis::from(col));
		}
	}

	features_analysis
}
