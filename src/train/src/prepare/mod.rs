mod normalize;
mod parse;

use std::collections::HashMap;

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
	let mut grouped_datasets = parse::datasets(&df);

	normalize::normalize(args, &df, &mut grouped_datasets);

	grouped_datasets
}
