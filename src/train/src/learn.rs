use float::Float;
use hypothesis::hypothesis;
use indicatif::ProgressIterator;
use model::Model;

use crate::{prepare::GroupedDatasets, Args};

pub fn learn(arg: &Args, grouped_datasets: &GroupedDatasets, mut model: Model) -> Model {
	for (label, datasets) in grouped_datasets.iter() {
		model.weights.insert(
			label.clone(),
			vec![0.0; datasets.training.first().expect("empty row").len()],
		);
	}

	for _ in (0..arg.iteration).progress() {
		for (label, thetas) in &mut model.weights {
			guess(thetas, grouped_datasets, arg.learning_rate, label);
		}
	}

	model
}

fn guess(thetas: &mut Vec<Float>, rows: &GroupedDatasets, learning_rate: Float, label: &str) {
	let mut sums: Vec<Float> = vec![0.0; thetas.len()];
	let mut count = 0;

	for (dataset_label, datasets) in rows {
		let truth = (label == dataset_label) as u8 as Float;

		for row in datasets.training.iter() {
			for (i, sum) in sums.iter_mut().enumerate() {
				*sum += (hypothesis(row, thetas) - truth) * row[i];
			}
		}

		count += datasets.training.len();
	}

	thetas.iter_mut().zip(sums.iter()).for_each(|(theta, sum)| {
		*theta -= learning_rate * (sum / count as Float);
	});
}
