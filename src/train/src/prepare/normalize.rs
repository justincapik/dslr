use analyze::Analysis;

use crate::{Args, Normalization};

use super::{Dataset, GroupedDatasets};

pub fn normalize(args: &Args, grouped_datasets: &mut GroupedDatasets, analysis: &[Analysis]) {
	for (_, datasets) in grouped_datasets.iter_mut() {
		normalize_dataset(args.normalization, &mut datasets.training, analysis);
		normalize_dataset(args.normalization, &mut datasets.testing, analysis);
	}
}

fn normalize_dataset(method: Normalization, dataset: &mut Dataset, analysis: &[Analysis]) {
	let expect = |r#type: &str| panic!("float feature must have a {type}");

	for row in dataset {
		for (feature, analysis) in row.iter_mut().zip(analysis.iter()) {
			*feature = match method {
				Normalization::MinMax => {
					let (min, max) = (
						analysis.min.unwrap_or_else(|| expect("min")),
						analysis.max.unwrap_or_else(|| expect("max")),
					);
					(*feature - min) / (max - min)
				}
				Normalization::StdDev => {
					let (mean, std_dev) = (
						analysis.mean.unwrap_or_else(|| expect("mean")),
						analysis
							.std
							.unwrap_or_else(|| expect("stdandard deviation")),
					);
					(*feature - mean) / std_dev
				}
			};
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use polars::prelude::*;

	#[test]
	fn test_normalize_min_max() {
		let method = Normalization::MinMax;

		let data = [("one", [1.0, 2.0, 3.0]), ("two", [2.0, 3.0, 4.0])];
		let mut dataset = (0..data[0].1.len())
			.map(|i| data.iter().map(|(_, values)| values[i]).collect::<Vec<_>>())
			.collect::<Vec<_>>();
		let analysis = data
			.iter()
			.map(|(name, values)| Analysis::from(Series::new((*name).into(), values)))
			.collect::<Vec<_>>();

		normalize_dataset(method, &mut dataset, &analysis);

		assert_eq!(
			dataset,
			vec![vec![0.0, 0.0], vec![0.5, 0.5], vec![1.0, 1.0]]
		);
	}

	#[test]
	fn test_normalize_std_dev() {
		let method = Normalization::StdDev;

		let data = [("one", [1.0, 3.0]), ("two", [2.0, 4.0])];
		let mut dataset = (0..data[0].1.len())
			.map(|i| data.iter().map(|(_, values)| values[i]).collect::<Vec<_>>())
			.collect::<Vec<_>>();
		let analysis = data
			.iter()
			.map(|(name, values)| Analysis::from(Series::new((*name).into(), values)))
			.collect::<Vec<_>>();

		normalize_dataset(method, &mut dataset, &analysis);

		assert_eq!(dataset, vec![vec![-1.0, -1.0], vec![1.0, 1.0]],);
	}
}
