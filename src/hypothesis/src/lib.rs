use float::Float;
use model::Model;

fn sigmoid(x: Float) -> Float {
	1.0 / (1.0 + (-x).exp())
}

pub fn hypothesis(x: &[Float], thetas: &[Float]) -> Float {
	sigmoid(
		x.iter()
			.zip(thetas.iter())
			.map(|(x, theta)| x * theta)
			.sum(),
	)
}

pub fn one_vs_all(x: &[Float], model: &Model) -> String {
	model
		.weights
		.iter()
		.map(|(label, thetas)| (label, hypothesis(x, thetas)))
		.max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
		.unwrap()
		.0
		.clone()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sigmod() {
		for (input, expected) in [
			(0.0, 0.5),
			(1.0, 0.7310586),
			(-1.0, 0.26894143),
			(0.5, 0.62245935),
		] {
			assert_eq!(sigmoid(input), expected);
		}
	}

	#[test]
	fn test_hypothesis() {
		let x = [-1.0, -0.5, 0.0, 0.5, 1.0];
		let y = [1.0, 0.5, 0.0, 0.5, 1.0];
		assert_eq!(hypothesis(&x, &y), 0.5);
	}

	#[test]
	fn test_one_vs_all() {
		let x = [-1.0, -0.5, 0.0, 0.5, 1.0];
		let model = Model {
			weights: vec![
				("a".to_string(), vec![1.0, 0.0, 0.0, 0.0, 0.0]),
				("b".to_string(), vec![0.0, 1.0, 0.0, 0.0, 0.0]),
				("c".to_string(), vec![0.0, 0.0, 1.0, 0.0, 0.0]),
				("d".to_string(), vec![0.0, 0.0, 0.0, 1.0, 0.0]),
				("e".to_string(), vec![0.0, 0.0, 0.0, 0.0, 1.0]),
			]
			.into_iter()
			.collect(),
			..Default::default()
		};

		let mut max = (String::new(), Float::MIN);
		for (label, thetas) in &model.weights {
			let h = hypothesis(&x, thetas);
			dbg!(label, h);
			if h > max.1 {
				max = (label.clone(), h);
			}
		}

		assert_eq!(one_vs_all(&x, &model), max.0);
		assert_eq!(one_vs_all(&x, &model), "e");
	}
}
