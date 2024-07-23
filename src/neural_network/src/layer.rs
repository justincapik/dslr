use crate::{Activation, Float, Matrix};

pub struct Layer {
	pub weight: Matrix,
	pub bias: Vec<Float>,
	pub activation: Activation,
}

impl Layer {
	// input • weight + bias -> activation = output
	pub fn forward(&self, input: &[Float]) -> Vec<Float> {
		assert_eq!(input.len(), self.weight.row);
		assert_eq!(self.weight.col, self.bias.len());

		let mut output = Vec::with_capacity(self.weight.col);

		for c in 0..self.weight.col {
			let mut sum = 0.0;

			#[allow(clippy::needless_range_loop)]
			for r in 0..self.weight.row {
				sum += input[r] * self.weight.data[r * self.weight.col + c];
			}

			output.push((self.activation)(sum + self.bias[c]));
		}

		output
	}

	// pub fn backward
}

#[cfg(test)]
mod tests {
	use std::vec;

	use crate::activation::relu;

	use super::*;

	#[test]
	fn test_layer() {
		let layer = Layer {
			weight: Matrix {
				row: 4,
				col: 5,
				data: vec![
					-1.0, -0.5, 0.0, 0.5, 1.0, //
					-1.0, -0.5, 0.0, 0.5, 1.0, //
					-1.0, -0.5, 0.0, 0.5, 1.0, //
					-1.0, -0.5, 0.0, 0.5, 1.0,
				],
			},
			bias: vec![-1.0, -0.5, 0.0, 0.5, 1.0],
			activation: relu,
		};

		let input = vec![-1.0, 0.0, 0.5, 1.0];
		let output = layer.process(&input);

		assert_eq!(output.len(), 5);
		assert_eq!(output, vec![0.0, 0.0, 0.0, 0.75, 1.5]);
	}
}
