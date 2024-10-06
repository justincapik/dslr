mod normalize;
mod parse;

use std::collections::HashMap;

use analyze::Analysis;
use float::Float;
use model::Model;
use polars::prelude::*;

use crate::{Args, Normalization};

pub type GroupedDatasets = HashMap<String, Datasets>;

#[derive(Debug, PartialEq)]
pub struct Datasets {
	pub training: Dataset,
	pub testing: Dataset,
}
pub type Dataset = Vec<Features>;
pub type Features = Vec<Float>;

pub fn prepare(args: &Args, df: DataFrame) -> (GroupedDatasets, Model) {
	let analysis = features_analysis(&df);

	let mut grouped_datasets = parse::datasets(&df, &analysis);

	let mut model = store_analysis(&analysis, args);
	model.label_name = label_name(&df);

	normalize::normalize(args, &mut grouped_datasets, &analysis);

	(grouped_datasets, model)
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

fn label_name(df: &DataFrame) -> String {
	df.get_columns()
		.iter()
		.find(|col| col.dtype().is_string())
		.expect("the datasets must have a string column for its label")
		.name()
		.to_string()
}

fn store_analysis(analysis: &[Analysis], args: &Args) -> Model {
	let expect = |r#type: &str| panic!("float feature must have a {type}");

	let mut model = Model::default();

	for feature_analysis in analysis {
		let mean = feature_analysis.mean.unwrap_or_else(|| expect("mean"));
		model.means.push(mean);

		match args.normalization {
			Normalization::MinMax => {
				let min = feature_analysis.min.unwrap_or_else(|| expect("min"));
				let max = feature_analysis.max.unwrap_or_else(|| expect("max"));

				model.normalization_factors.push((min, min - max));
			}
			Normalization::StdDev => {
				let std = feature_analysis
					.std
					.unwrap_or_else(|| expect("stdandard deviation"));

				model.normalization_factors.push((mean, std));
			}
		}
	}

	model
}
