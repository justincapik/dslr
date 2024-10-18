use plotly::{
	color::NamedColor,
	common::{Marker, Mode},
	Histogram, Scatter,
};

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

pub fn scatter(
	t: Vec<f64>,
	y: Vec<f64>,
	label: &str,
	color: NamedColor,
	plot_index: usize,
) -> Box<Scatter<f64, f64>> {
	Scatter::new(t, y)
		.mode(Mode::Markers)
		.marker(Marker::new().color(color).size(3))
		.x_axis(format!("x{plot_index}"))
		.y_axis(format!("y{plot_index}"))
		.name(label)
}