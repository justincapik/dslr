use crate::PlotType;

use super::LABEL_NAME;
use plotly::{
	color::NamedColor,
	layout::{GridPattern, ItemSizing, LayoutGrid, Legend},
	Layout,
};

pub fn build(plot_type: PlotType) -> Layout {
	Layout::new()
		.grid(
			LayoutGrid::new()
				.rows(4)
				.columns(4)
				.pattern(GridPattern::Independent),
		)
		.legend(
			Legend::new()
				.title(LABEL_NAME)
				.item_sizing(ItemSizing::Constant)
				.border_color(NamedColor::Black)
				.border_width(1)
				.background_color(NamedColor::GhostWhite),
		)
		.title(format!("{plot_type} plot of {LABEL_NAME}"))
}
