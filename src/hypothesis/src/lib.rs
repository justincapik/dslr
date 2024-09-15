use float::Float;

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
}
