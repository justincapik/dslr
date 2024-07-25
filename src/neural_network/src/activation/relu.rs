use crate::Float;

use super::Activation;

pub struct ReLU;

impl Activation for ReLU {
	fn forward(x: Float) -> Float {
		if x > 0.0 {
			x
		} else {
			0.0
		}
	}

	fn backward(x: Float) -> Float {
		if x > 0.0 {
			1.0
		} else {
			0.0
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_relu_forward() {
		for (input, expected) in [
			(0.0, 0.0),
			(1.0, 1.0),
			(-1.0, 0.0),
			(0.5, 0.5),
			(-0.5, 0.0),
			(0.1, 0.1),
		] {
			assert_eq!(ReLU::forward(input), expected);
		}
	}

	#[test]
	fn test_relu_backward() {
		for (input, expected) in [
			(0.0, 0.0),
			(1.0, 1.0),
			(-1.0, 0.0),
			(0.5, 1.0),
			(-0.5, 0.0),
			(0.1, 1.0),
		] {
			assert_eq!(ReLU::backward(input), expected);
		}
	}
}
