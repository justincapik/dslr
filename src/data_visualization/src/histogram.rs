use std::{error::Error, path::Path};

use plotly::{common::Marker, Histogram, Plot};
use polars::prelude::*;

use visualize::{annotation, args, feature, image, layout, populate, Label, PlotType, LABEL_NAME};

fn main() -> Result<(), Box<dyn Error>> {
	let args = args::parse("histogram.png");

	let mut dataset = load::load(args.csv)?;

	populate::date(&mut dataset)?;

	plot(dataset, args.output)
}

fn plot<P: AsRef<Path>>(dataset: DataFrame, output: P) -> Result<(), Box<dyn Error>> {
	let mut plot = Plot::new();

	let mut layout = layout::build(PlotType::Histogram);

	for df_label in dataset.partition_by([LABEL_NAME], true)? {
		let mut plot_index = 1;

		let label = df_label
			.column(LABEL_NAME)
			.expect("Label column not found")
			.iter()
			.next()
			.expect("Series is empty");
		let label: Label = label.get_str().unwrap_or("").parse()?;
		let color = label.color();
		let label = label.to_string();

		for series in df_label.get_columns() {
			let Ok(col) = feature::parse(series) else {
				continue;
			};

			plot.add_trace(
				Histogram::new(col)
					.marker(Marker::new().color(color))
					.x_axis(format!("x{plot_index}"))
					.y_axis(format!("y{plot_index}"))
					.name(&label),
			);

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
