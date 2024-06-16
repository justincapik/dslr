#![allow(dead_code)]
#![allow(unused)]

use itertools_num::linspace;
use plotly::common::{
	self, ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::Scatter;
use plotly::{ImageFormat, Plot};

use plotly::box_plot::{BoxMean, BoxPoints};
use plotly::color::{NamedColor, Rgb, Rgba};
use plotly::common::{ErrorData, ErrorType, Orientation};
use plotly::histogram::{Bins, Cumulative, HistFunc, HistNorm};
use plotly::layout::{BoxMode, Margin};
use plotly::{Bar, BoxPlot, Histogram};
use rand;
use rand_distr::{Distribution, Normal, Uniform};

// Histograms
fn sample_normal_distribution(n: usize, mean: f64, std_dev: f64) -> Vec<f64> {
	let mut rng = rand::thread_rng();
	let dist = Normal::new(mean, std_dev).unwrap();
	let mut v = Vec::<f64>::with_capacity(n);
	for _idx in 1..n {
		v.push(dist.sample(&mut rng));
	}
	v
}

pub fn simple_scatter_plot(col: Vec<f64>) {
	let min = 0.0;
	let max = col.len() as f64;
	let n = (max - min) as usize;
	let t: Vec<f64> = linspace(min, max, n).collect();
	//let y = t.iter().map(|x| x.sin()).collect();
	let y = col;

	//let samples = sample_normal_distribution(10_000, 0.0, 1.0);
	let trace = Scatter::new(t, y).mode(Mode::Markers);
	let mut plot = Plot::new();
	plot.add_trace(trace);
	plot.write_image("scatter.png", ImageFormat::PNG, 800, 600, 1.0);
	//plot.show();
}

pub fn basic_histogram() {
	let samples1 = sample_normal_distribution(500, 0.0, 1.0);
	let trace1 = Histogram::new(samples1)
		.name("trace 1")
		.opacity(0.5)
		.marker(Marker::new().color(NamedColor::Green));

	let samples2 = sample_normal_distribution(500, 0.0, 1.0);
	let trace2 = Histogram::new(samples2)
		.name("trace 2")
		.opacity(0.6)
		.marker(Marker::new().color(NamedColor::Red));

	let mut plot = Plot::new();
	plot.add_trace(trace1);
	plot.add_trace(trace2);

	let layout = Layout::new().bar_mode(BarMode::Overlay);
	plot.set_layout(layout);

	plot.write_image("histo.png", ImageFormat::PNG, 800, 600, 1.0);
	//plot.show();
}
