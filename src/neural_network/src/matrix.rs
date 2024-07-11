use crate::Float;

pub struct Matrix {
	pub row: usize, // input interface
	pub col: usize, // output interface
	pub data: Vec<Float>,
}
