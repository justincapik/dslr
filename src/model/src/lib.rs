use std::{collections::HashMap, path::Path};

use float::Float;

#[derive(Default)]
pub struct Model {
	pub weights: HashMap<String, Vec<Float>>,
	pub normalization_factors: Vec<(Float, Float)>,
}

impl Model {
	pub fn write(&self, path: &Path) -> std::io::Result<()> {
		let mut wtr = csv::Writer::from_path(path)?;

		let record_len = 2 + self.weights.len();

		let mut header = Vec::with_capacity(record_len);
		header.push("s");
		header.push("k");

		let mut series: Vec<&[Float]> = Vec::with_capacity(self.weights.len());
		for (label, thetas) in &self.weights {
			header.push(label);
			series.push(thetas);

			assert_eq!(thetas.len(), self.normalization_factors.len());
		}

		wtr.write_record(header)?;

		for i in 0..self.normalization_factors.len() {
			let mut record: Vec<String> = Vec::with_capacity(record_len);

			record.push(self.normalization_factors[i].0.to_string());
			record.push(self.normalization_factors[i].1.to_string());

			for thetas in &series {
				record.push(thetas[i].to_string());
			}

			wtr.write_record(record)?;
		}

		wtr.flush()?;
		Ok(())
	}

	pub fn read(path: &Path) -> Self {
		todo!()
	}
}
