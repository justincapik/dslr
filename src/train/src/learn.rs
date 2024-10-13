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
			let mut sums: Vec<Float> = vec![0.0; thetas.len()];
			let mut count = 0;

			for (dataset_label, datasets) in grouped_datasets {
				let truth: Float = if label == dataset_label { 1.0 } else { 0.0 };

				for row in datasets.training.iter() {
					for (i, sum) in sums.iter_mut().enumerate() {
						*sum += (hypothesis(row, thetas) - truth) * row[i];
					}
				}

				count += datasets.training.len();
			}

			thetas.iter_mut().zip(sums.iter()).for_each(|(theta, sum)| {
				*theta -= arg.learning_rate * (sum / count as Float);
			});
		}
	}

	model
}

/*
fn guess(thetas: &[Float], rows: GroupedDatasets, learning_rate: Float, truth: bool) -> Vec<Float> {
	let mut new_thetas = thetas.to_vec();
	let truth: Float = if truth { 1.0 } else { 0.0 };

	for (i, new_theta) in new_thetas.iter_mut().enumerate() {
		let sum = rows
			.iter()
			.map(|row| (hypothesis(row, thetas).powi(2) - truth.powi(2)) * row[i])
			.sum::<Float>();

		*new_theta -= learning_rate * (sum / rows.len() as Float);
	}

	new_thetas
}
*/
