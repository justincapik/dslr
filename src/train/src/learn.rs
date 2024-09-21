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
			for (model_label, thetas) in &mut model.weights {
				*thetas = guess(
					thetas,
					&datasets.training,
					arg.learning_rate,
					label == model_label,
				);
			}
			// let thetas = model.weights.get_mut(label).expect("label not found");
			// *thetas = guess(thetas, &datasets.training, arg.learning_rate);
		}
	}

	// debug
	for (label, thetas) in &model.weights {
		dbg!(label, thetas);
	}
	dbg_precision(&grouped_datasets, &model);

	model
}

fn guess(
	thetas: &[Float],
	rows: &Vec<Vec<Float>>,
	learning_rate: Float,
	truth: bool,
) -> Vec<Float> {
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

fn dbg_precision(grouped_datasets: &GroupedDatasets, model: &Model) {
	for (label, datasets) in grouped_datasets.iter() {
		let mut correct = 0;
		let mut total = 0;

		for row in &datasets.testing {
			let prediction = one_vs_all(row, &model);
			// let mut max = (String::new(), Float::MIN);
			// for (label, thetas) in &model.weights {
			// 	let h = hypothesis(row, thetas);
			// 	dbg!(label, h);
			// 	if h > max.1 {
			// 		max = (label.clone(), h);
			// 	}
			// }
			if &prediction == label {
				correct += 1;
			}
			total += 1;
		}

		println!(
			"{label}:\t\x1b[1;32m{correct}\x1b[0m/\x1b[1m{total}\x1b[0m\t\x1b[1;35m{percent:.2}%\x1b[0m",
			percent = correct as Float / total as Float * 100.0
		);

		let mut correct = 0;
		let mut total = 0;

		for row in &datasets.training {
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
}
