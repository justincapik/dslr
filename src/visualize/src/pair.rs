use std::{error::Error, path::Path};

use plotly::Plot;
use polars::prelude::*;

use visualize::{
	annotation, args, feature, image, layout, populate, trace, Label, PlotType, LABEL_NAME,
};

const USEFUL_FEATURES: [&str; 6] = [
	"Divination",
	"Ancient Runes",
	"Potions",
	"History of Magic",
	"Charms",
	"Flying",
];

fn main() -> Result<(), Box<dyn Error>> {
	let args = args::parse("pair_plot.png");

	let mut dataset = load::load(args.csv)?;

	populate::date(&mut dataset)?;

	plot(dataset, args.output)
}

fn plot<P: AsRef<Path>>(dataset: DataFrame, output: P) -> Result<(), Box<dyn Error>> {
	let mut plot = Plot::new();

	let mut layout = layout::build(PlotType::Pair, USEFUL_FEATURES.len());

	for (i, df_label) in dataset
		.partition_by([LABEL_NAME], true)?
		.into_iter()
		.enumerate()
	{
		let mut plot_index = 1;

		let (label, color) = Label::extract(&df_label)?;

		for series_y in df_label.get_columns() {
			let name_y = series_y.name();
			if !USEFUL_FEATURES.contains(&name_y.as_str()) {
				continue;
			}

			for series_x in df_label.get_columns() {
				let name_x = series_x.name();
				if !USEFUL_FEATURES.contains(&name_x.as_str()) {
					continue;
				}

				let Some(y) = feature::parse(series_y) else {
					continue;
				};

				if name_x == name_y {
					plot.add_trace(trace::histogram(y, &label, color, plot_index));
				} else {
					let Some(x) = feature::parse(series_x) else {
						continue;
					};

					plot.add_trace(trace::scatter(x, y, &label, color, plot_index));
				}

				if i == 0 {
					layout.add_annotation(annotation(plot_index, &format!("{name_y} vs {name_x}")));
				}

				plot_index += 1;
			}
		}
	}

	plot.set_layout(layout);

	plot.write_image(
		output,
		image::FORMAT,
		((image::WIDTH as f64) * 1.25) as usize,
		((image::HEIGHT as f64) * 1.25) as usize,
		image::SCALE,
	);

	Ok(())
}
