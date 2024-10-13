use std::collections::HashMap;

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
		let mut new_thetas: HashMap<String, Vec<Float>> = HashMap::new();

		for (label, datasets) in grouped_datasets {
			for (model_label, thetas) in &model.weights {
				let updated_thetas = guess(
					thetas,
					&datasets.training,
					arg.learning_rate,
					label == model_label,
				);

				let entry = new_thetas
					.entry(model_label.to_owned())
					.or_insert_with(|| vec![0.0; thetas.len()]);
				entry
					.iter_mut()
					.zip(updated_thetas.iter())
					.for_each(|(a, b)| *a += b);
			}
		}

		for (label, thetas) in new_thetas {
			let thetas = thetas
				.iter()
				.map(|theta| theta / grouped_datasets.len() as Float)
				.collect();
			model.weights.insert(label, thetas);
		}
	}

	model
}

fn guess(thetas: &[Float], rows: &[Vec<Float>], learning_rate: Float, truth: bool) -> Vec<Float> {
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
