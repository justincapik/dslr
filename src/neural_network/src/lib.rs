mod activation;
mod layer;
mod matrix;

use activation::Activation;
use layer::Layer;
use matrix::Matrix;

type Float = f32;

pub struct NeuralNetwork {
	layers: Vec<Layer>,
}

impl NeuralNetwork {
	pub fn forward(&self, input: Vec<Float>) -> Vec<Float> {
		let mut output = input;

		for layer in &self.layers {
			output = layer.forward(&output);
		}

		output
	}
}
