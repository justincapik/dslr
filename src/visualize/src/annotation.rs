use plotly::{common::Anchor, layout::Annotation};

pub fn annotation(plot_index: usize, series_name: &str) -> Annotation {
	Annotation::new()
		.y(1.1)
		.x(0.5)
		.y_ref(format!("y{plot_index} domain"))
		.x_ref(format!("x{plot_index} domain"))
		.y_anchor(Anchor::Top)
		.x_anchor(Anchor::Center)
		.show_arrow(false)
		.text(series_name)
}
