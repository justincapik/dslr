use std::{collections::HashMap, path::Path};

use float::Float;

#[derive(Default)]
pub struct Model {
	pub weights: HashMap<String, Vec<Float>>,
}

impl Model {
	pub fn write(&self, path: &Path) {
		todo!()
	}

	pub fn read(path: &Path) -> Self {
		todo!()
	}
}
