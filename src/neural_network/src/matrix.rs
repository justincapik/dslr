use crate::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
	pub row: usize, // input interface
	pub col: usize, // output interface
	pub data: Vec<Float>,
}

impl Matrix {
	pub fn new(row: usize, col: usize) -> Self {
		Self {
			row,
			col,
			data: vec![0.0; row * col],
		}
	}
}
