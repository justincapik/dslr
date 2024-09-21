use float::Float;
use hypothesis::hypothesis;
use indicatif::ProgressIterator;
use model::Model;

use crate::{prepare::GroupedDatasets, Args};

pub fn learn(arg: &Args, grouped_datasets: &GroupedDatasets) -> Model {
	let mut model = Model::default();

	for (label, datasets) in grouped_datasets.iter() {
		model.weights.insert(
			label.clone(),
			vec![0.0; datasets.training.first().expect("empty row").len()],
		);
	}

	for _ in (0..arg.iteration).progress() {
		for (label, datasets) in grouped_datasets {
			for (model_label, thetas) in &mut model.weights {
				*thetas = guess(
					thetas,
					&datasets.training,
					arg.learning_rate,
					label == model_label,
				);
			}
		}
	}

	model
}

fn guess(thetas: &[Float], rows: &[Vec<Float>], learning_rate: Float, truth: bool) -> Vec<Float> {
	let mut new_thetas = thetas.to_vec();
	let truth = if truth { 1.0 } else { 0.0 };

	for (i, new_theta) in new_thetas.iter_mut().enumerate() {
		let sum = rows
			.iter()
			.map(|row| (hypothesis(row, thetas) - truth) * row[i])
			.sum::<Float>();

		*new_theta -= learning_rate * (sum / rows.len() as Float);
	}

	new_thetas
}
