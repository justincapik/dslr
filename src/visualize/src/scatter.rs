use std::{error::Error, path::Path};

use plotly::Plot;
use polars::prelude::*;

use visualize::{
	annotation, args, feature, image, layout, populate, trace, Label, PlotType, LABEL_NAME,
};

fn main() -> Result<(), Box<dyn Error>> {
	let args = args::parse("scatter_plot.png");

	let mut dataset = load::load(args.csv)?;

	populate::date(&mut dataset)?;

	plot(dataset, args.output)
}

fn plot<P: AsRef<Path>>(dataset: DataFrame, output: P) -> Result<(), Box<dyn Error>> {
	let mut plot = Plot::new();

	let mut layout = layout::build(PlotType::Scatter, 4);

	for df_label in dataset.partition_by([LABEL_NAME], true)? {
		let mut plot_index = 1;

		let (label, color) = Label::extract(&df_label)?;

		for series in df_label.get_columns() {
			let Ok(y) = feature::parse(series) else {
				continue;
			};

			let t: Vec<f64> = y.iter().copied().rev().collect();

			plot.add_trace(trace::scatter(t, y, &label, color, plot_index));

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
