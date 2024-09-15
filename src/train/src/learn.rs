use float::Float;
use hypothesis::{hypothesis, one_vs_all};
use indicatif::ProgressIterator;
use model::Model;

use crate::{prepare::GroupedDatasets, Args};

pub fn learn(arg: &Args, grouped_datasets: GroupedDatasets) -> Model {
	let mut model = Model::default();

	for (label, datasets) in grouped_datasets.iter() {
		model.weights.insert(
			label.clone(),
			vec![0.0; datasets.training.get(0).expect("empty row").len()],
		);
	}

	for _ in (0..arg.iteration).progress() {
		for (label, datasets) in &grouped_datasets {
			let thetas = model.weights.get_mut(label).expect("label not found");
			*thetas = guess(thetas, &datasets.training, arg.learning_rate);
		}
	}

	// debug
	for (label, thetas) in &model.weights {
		dbg!(label, thetas);
	}

	for (label, datasets) in grouped_datasets.iter() {
		let mut correct = 0;
		let mut total = 0;

		for row in &datasets.testing {
			let prediction = one_vs_all(row, &model);
			if &prediction == label {
				correct += 1;
			}
			total += 1;
		}

		println!(
			"{label}:\t\x1b[1;32m{correct}\x1b[0m/\x1b[1m{total}\x1b[0m\t\x1b[1;35m{percent:.2}%\x1b[0m",
			percent = correct as Float / total as Float * 100.0
		);
	}

	model
}

fn guess(thetas: &[Float], rows: &Vec<Vec<Float>>, learning_rate: Float) -> Vec<Float> {
	let mut new_thetas = thetas.to_vec();

	for (new_theta, row) in new_thetas.iter_mut().zip(rows.iter()) {
		*new_theta -= learning_rate * hypothesis(row, thetas);
	}

	new_thetas
}
