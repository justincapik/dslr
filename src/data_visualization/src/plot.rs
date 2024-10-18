pub fn plot<P: AsRef<Path>>(dataset: DataFrame, output: P) -> Result<(), Box<dyn Error>> {
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
			let Ok(feature) = series.f64() else {
				continue;
			};

			let col: Vec<f64> = feature
				.into_iter()
				.flatten()
				.filter(|x| *x != 0.0)
				.collect::<Vec<f64>>();

			if col.is_empty() {
				continue;
			}

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
