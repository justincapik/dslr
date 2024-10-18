use std::{error::Error, path::Path};

use plotly::Plot;
use polars::prelude::*;

use visualize::{
	annotation, args, feature, image, layout, populate, trace, Label, PlotType, LABEL_NAME,
};

fn main() -> Result<(), Box<dyn Error>> {
	let args = args::parse("histogram.png");

	let mut dataset = load::load(args.csv)?;

	populate::date(&mut dataset)?;

	plot(dataset, args.output)
}

fn plot<P: AsRef<Path>>(dataset: DataFrame, output: P) -> Result<(), Box<dyn Error>> {
	let mut plot = Plot::new();

	let mut layout = layout::build(PlotType::Histogram, 4);

	for df_label in dataset.partition_by([LABEL_NAME], true)? {
		let mut plot_index = 1;

		let (label, color) = Label::extract(&df_label)?;

		for series in df_label.get_columns() {
			let Ok(col) = feature::parse(series) else {
				continue;
			};

			plot.add_trace(trace::histogram(col, &label, color, plot_index));

			layout.add_annotation(annotation(plot_index, &label));

			plot_index += 1;
		}
	}

	plot.set_layout(layout);

	plot.write_image(
		output,
		image::FORMAT,
		image::WIDTH,
		image::HEIGHT,
		image::SCALE,
	);

	Ok(())
}
