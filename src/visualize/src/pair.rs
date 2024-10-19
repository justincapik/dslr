use std::{error::Error, path::PathBuf};

use plotly::Plot;
use polars::prelude::*;

use visualize::{annotation, args, feature, image, layout, trace, Label, PlotType, LABEL_NAME};

const USEFUL_FEATURES: [&str; 6] = [
	"Divination",
	"Ancient Runes",
	"Potions",
	"History of Magic",
	"Charms",
	"Flying",
];
const EXPECTED_FEATURES_LEN: usize = 13;

fn main() -> Result<(), Box<dyn Error>> {
	let args = args::parse("pair_plot.png");

	let dataset = load::load(args.csv)?;

	plot(&dataset, args.output.clone(), false)?;
	plot(&dataset, args.output, true)?;

	Ok(())
}

fn plot(dataset: &DataFrame, output: PathBuf, all_features: bool) -> Result<(), Box<dyn Error>> {
	let mut plot = Plot::new();

	let size = if all_features {
		EXPECTED_FEATURES_LEN
	} else {
		USEFUL_FEATURES.len()
	};
	let mut layout = layout::build(PlotType::Pair, size);

	for (i, df_label) in dataset
		.partition_by([LABEL_NAME], true)?
		.into_iter()
		.enumerate()
	{
		let mut plot_index = 1;

		let (label, color) = Label::extract(&df_label)?;

		for series_y in df_label.get_columns() {
			let name_y = series_y.name();
			if !all_features && !USEFUL_FEATURES.contains(&name_y.as_str()) {
				continue;
			}

			for series_x in df_label.get_columns() {
				let name_x = series_x.name();
				if !all_features && !USEFUL_FEATURES.contains(&name_x.as_str()) {
					continue;
				}

				let Some(y) = feature::parse(series_y) else {
					continue;
				};

				if name_x == name_y {
					plot.add_trace(trace::histogram(y, &label, color, plot_index));

					if i == 0 {
						layout.add_annotation(annotation(plot_index, name_y));
					}
				} else {
					let Some(x) = feature::parse(series_x) else {
						continue;
					};

					plot.add_trace(trace::scatter(x, y, &label, color, plot_index));

					if i == 0 {
						layout.add_annotation(annotation(
							plot_index,
							&format!("{name_y} vs {name_x}"),
						));
					}
				}

				plot_index += 1;
			}
		}
	}

	plot.set_layout(layout);

	plot.write_image(
		get_output(output, all_features),
		image::FORMAT,
		((image::WIDTH as f64) / 4.0 * size as f64) as usize,
		((image::HEIGHT as f64) / 4.0 * size as f64) as usize,
		image::SCALE,
	);

	Ok(())
}

fn get_output(output: PathBuf, all_features: bool) -> PathBuf {
	if !all_features {
		return output;
	}

	let Some(filename) = output.file_stem() else {
		return output;
	};

	PathBuf::from(format!(
		"{}_complete.{}",
		filename.to_string_lossy(),
		output
			.extension()
			.unwrap_or_else(|| "".as_ref())
			.to_string_lossy()
	))
}
