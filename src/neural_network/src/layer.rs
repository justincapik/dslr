use crate::{activation, Activation, Float, Matrix};

pub struct Layer {
	pub weight: Matrix,
	pub bias: Vec<Float>,
}

pub struct Cache<A: Activation> {
	pub input: Vec<Float>,
	pub weight: Matrix,
	pub bias: Vec<Float>,
	pub pre_activation: Vec<Float>,
	pub activation: A,
}

impl Layer {
	/// input • weight + bias -> activation = output
	fn modular_forward<T>(
		&self,
		input: &[Float],
		mut activation: impl FnMut(Float) -> T,
	) -> Vec<T> {
		assert_eq!(input.len(), self.weight.row);
		assert_eq!(self.weight.col, self.bias.len());

		let mut output = Vec::with_capacity(self.weight.col);

		for c in 0..self.weight.col {
			let mut sum = 0.0;

			#[allow(clippy::needless_range_loop)]
			for r in 0..self.weight.row {
				sum += input[r] * self.weight.data[r * self.weight.col + c];
			}

			output.push(activation(sum + self.bias[c]));
		}

		output
	}

	/// input • weight + bias -> activation = output
	#[inline]
	pub fn forward<A: Activation>(&self, input: &[Float]) -> Vec<Float> {
		return self.modular_forward(input, A::forward);
	}

	pub fn forward_cache<A: Activation>(
		&self,
		input: Vec<Float>,
		activation: A,
	) -> (Vec<Float>, Cache<A>) {
		let mut pre_activation = Vec::with_capacity(self.weight.col);
		let custom_activation = |x: Float| {
			pre_activation.push(x);
			A::forward(x)
		};

		let output = self.modular_forward(&input, custom_activation);

		(
			output,
			Cache {
				input,
				weight: self.weight.clone(),
				bias: self.bias.clone(),
				pre_activation,
				activation,
			},
		)
	}

	// pub fn backward
}

#[cfg(test)]
mod tests {
	use std::vec;

	use crate::activation::ReLU;

	use super::*;

	#[test]
	fn test_forward() {
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
		};

		let input = vec![-1.0, 0.0, 0.5, 1.0];
		let output = layer.forward::<ReLU>(&input);

		assert_eq!(output.len(), 5);
		assert_eq!(output, vec![0.0, 0.0, 0.0, 0.75, 1.5]);
	}

	#[test]
	fn test_forward_cache() {
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
		};

		let input = vec![-1.0, 0.0, 0.5, 1.0];
		let (output, cache) = layer.forward_cache(input.clone(), ReLU);

		assert_eq!(output, vec![0.0, 0.0, 0.0, 0.75, 1.5]);

		assert_eq!(cache.input, input);
		assert_eq!(cache.weight, layer.weight);
		assert_eq!(cache.bias, layer.bias);
		assert_eq!(cache.pre_activation, vec![-1.5, -0.75, 0.0, 0.75, 1.5]);
	}
}
