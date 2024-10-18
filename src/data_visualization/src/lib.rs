pub mod args;
pub mod feature;
pub mod layout;
pub mod populate;

mod annotation;
pub use annotation::annotation;

mod label;
pub use label::Label;

mod plot_type;
pub use plot_type::PlotType;

pub const LABEL_NAME: &str = "Hogwarts House";

pub mod image {
	pub const FORMAT: plotly::ImageFormat = plotly::ImageFormat::PNG;
	pub const WIDTH: usize = 1200;
	pub const HEIGHT: usize = 1200;
	pub const SCALE: f64 = 1.0;
}
