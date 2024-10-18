pub fn write_trace(
	name: &str,
	house_name: &str,
	i: usize,
	one_house: DataFrame,
	legend_check: &mut usize,
	plot: &mut Plot,
	layout: &mut Layout,
) -> Result<(), ()> {
	let series = one_house.column(name).unwrap();
	let annot = Annotation::new()
		.y(1.1)
		.x(0.5)
		.y_ref(format!("y{i} domain"))
		.x_ref(format!("x{i} domain"))
		.y_anchor(Anchor::Top)
		.x_anchor(Anchor::Center)
		.show_arrow(false)
		.text(name);

	match make_trace(i, series, house_name) {
		Err(_) => (),
		Ok(mut trace) => {
			if *legend_check < 4 {
				trace = trace.name(house_name);
				*legend_check += 1;
			} else {
				trace = trace.show_legend(false);
			}
			plot.add_trace(trace);
			(*layout).add_annotation(annot);
		}
	};
}