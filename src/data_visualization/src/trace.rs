use plotly::{color::NamedColor, common::Marker, Histogram};

pub fn histogram(
	col: Vec<f64>,
	label: &str,
	color: NamedColor,
	plot_index: usize,
) -> Box<Histogram<f64>> {
	Histogram::new(col)
		.marker(Marker::new().color(color))
		.x_axis(format!("x{plot_index}"))
		.y_axis(format!("y{plot_index}"))
		.name(label)
}
