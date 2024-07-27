use float::Float;
use hypothesis::hypothesis;
use indicatif::ProgressIterator;
use model::Model;

use crate::{group::GroupedRow, Args};

pub fn learn(arg: &Args, grouped_row: GroupedRow) {
	let mut model = Model::default();

	for (label, rows) in grouped_row.iter() {
		model.weights.insert(
			label.clone(),
			vec![0.0; rows.get(0).expect("empty row").len()],
		);
	}

	for _ in (0..arg.iteration).progress() {
		for (label, rows) in &grouped_row {
			let thetas = model.weights.get_mut(label).expect("label not found");
			*thetas = guess(thetas, rows, arg.learning_rate);
		}
	}
}

fn guess(thetas: &[Float], rows: &Vec<Vec<Float>>, learning_rate: Float) -> Vec<Float> {
	let mut new_thetas = thetas.to_vec();

	for (new_theta, ) in  {
		new_thetas[i] -= learning_rate * hypothesis(row, thetas);
	}

	new_thetas
}
