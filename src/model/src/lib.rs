use hmerr::{ioe, se};
use std::{collections::HashMap, path::Path};

use float::Float;

#[derive(Default)]
pub struct Model {
	pub label_name: String,
	pub weights: HashMap<String, Vec<Float>>,
	pub normalization_factors: Vec<(Float, Float)>,
}

const FACTOR_AMOUNT: usize = 2;

impl Model {
	pub fn write(&self, path: &Path) -> std::io::Result<()> {
		let mut wtr = csv::Writer::from_path(path)?;

		let record_len = FACTOR_AMOUNT + self.weights.len();

		let mut header = Vec::with_capacity(record_len);
		header.push("k");
		header.push(&self.label_name);

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

	pub fn read(path: &Path) -> hmerr::Result<Self> {
		let mut rdr = csv::Reader::from_path(path).map_err(|e| ioe!(path.to_string_lossy(), e))?;

		let mut model = Model::default();

		let mut header = rdr
			.headers()
			.map_err(|e| ioe!(path.to_string_lossy(), e))?
			.iter()
			.map(|s| s.to_string())
			.skip(FACTOR_AMOUNT - 1);

		model.label_name = header
			.next()
			.expect(format!("{path} has incorrect header", path = path.to_string_lossy()).as_str())
			.to_string();

		let header = header.collect::<Vec<_>>();

		for label in header.iter() {
			model.weights.insert(label.to_string(), Vec::new());
		}

		let parse_cell = |cell: Option<&str>| {
			let Some(cell) = cell else {
				return Err(se!(
					format!(
						"could not \x1b[1;31mparse\x1b[0m \x1b[1;33mmodel ({path})\x1b[0m",
						path = path.to_string_lossy()
					),
					"float",
					"empty cell",
				));
			};

			cell.parse().map_err(|e| {
				se!(
					format!("could not \x1b[1;31mparse\x1b[0m \x1b[1;33mmodel ({path})\x1b[0m", path=path.to_string_lossy()),
					"float",
					cell,
					s:e,
				)
			})
		};

		for result in rdr.records() {
			let record = result.map_err(|e| ioe!(path.to_string_lossy(), e))?;

			let mut record = record.iter();

			model
				.normalization_factors
				.push((parse_cell(record.next())?, parse_cell(record.next())?));

			for label in header.iter() {
				model
					.weights
					.get_mut(label)
					.expect("label disappeared?")
					.push(parse_cell(record.next())?);
			}
		}

		Ok(model)
	}
}

#[cfg(test)]
mod test {
	use std::path::Path;

	#[test]
	fn test_model() {
		let model = super::Model {
			label_name: "some cool name".to_string(),
			weights: vec![
				("a".to_string(), vec![0.0, 1.0]),
				("b".to_string(), vec![2.0, 3.0]),
			]
			.into_iter()
			.collect(),
			normalization_factors: vec![(1.0, 2.0), (3.0, 4.0)],
		};

		let path = Path::new("/tmp/cargo_test_dslr_model.csv");

		model.write(&path).unwrap();

		let read_model = super::Model::read(&path).unwrap();

		assert_eq!(model.label_name, read_model.label_name);
		assert_eq!(model.weights, read_model.weights);
		assert_eq!(
			model.normalization_factors,
			read_model.normalization_factors
		);
	}
}
