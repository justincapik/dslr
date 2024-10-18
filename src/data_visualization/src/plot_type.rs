#[derive(Debug, Clone, Copy)]
pub enum PlotType {
	Histogram,
	Scatter,
	Pair,
}

impl std::fmt::Display for PlotType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			PlotType::Histogram => write!(f, "histogram"),
			PlotType::Scatter => write!(f, "scatter"),
			PlotType::Pair => write!(f, "pair"),
		}
	}
}
