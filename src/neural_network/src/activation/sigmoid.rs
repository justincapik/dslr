use crate::Float;

use super::Activation;

pub struct Sigmoid;

impl Activation for Sigmoid {
	fn forward(x: Float) -> Float {
		1.0 / (1.0 + (-x).exp())
	}

	fn backward(x: Float) -> Float {
		let s = Self::forward(x);
		s * (1.0 - s)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sigmoid_forward() {
		for (input, expected) in [
			(0.0, 0.5),
			(1.0, 0.7310586),
			(-1.0, 0.26894143),
			(0.5, 0.62245935),
		] {
			assert_eq!(Sigmoid::forward(input), expected);
		}
	}

	#[test]
	fn test_sigmoid_backward() {
		for (input, expected) in [
			(0.0, 0.25),
			(1.0, 0.19661193),
			(-1.0, 0.19661193),
			(0.5, 0.23500371),
		] {
			assert_eq!(Sigmoid::backward(input), expected);
		}
	}
}
