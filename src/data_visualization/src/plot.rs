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
fn sample_uniform_distribution(n: usize, lb: f64, ub: f64) -> Vec<f64> {
	let mut rng = rand::thread_rng();
	let dist = Uniform::new(lb, ub);
	let mut v = Vec::<f64>::with_capacity(n);
	for _idx in 1..n {
		v.push(dist.sample(&mut rng));
	}
	v
}

pub fn simple_scatter_plot(flying: Vec<f64>, potions: Vec<f64>, charms: Vec<f64>) {
	let mut plot = Plot::new();

	for col in vec![flying, potions, charms] {
		let min = 0.0;
		let max = col.len() as f64;
		let n = (max - min) as usize;
		let t: Vec<f64> = col.iter().copied().rev().collect();
		let y = col;

		let trace = Scatter::new(t, y).mode(Mode::Markers);
		plot.add_trace(trace);
	}
	plot.write_image("data_scatter.png", ImageFormat::PNG, 800, 600, 1.0);
}

pub fn histogram_2_1(houses: Vec<Vec<f64>>, house_names: Vec<String>) {
	let mut plot = Plot::new();

	for (vals, name) in houses.into_iter().zip(house_names.into_iter()) {
		let trace = Histogram::new(vals)
			.name(name)
			.opacity(0.5)
			.marker(Marker::new().color(NamedColor::Green));

		plot.add_trace(trace);
	}

	let layout = Layout::new().bar_mode(BarMode::Overlay);
	plot.set_layout(layout);

	plot.write_image("histo_2_1.png", ImageFormat::PNG, 800, 600, 1.0);
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

pub fn normalized_histogram() {
	let n = 500;
	let x = sample_uniform_distribution(n, 0.0, 1.0);
	let trace = Histogram::new(x)
		.hist_norm(HistNorm::Probability)
		.marker(Marker::new().color(NamedColor::SeaGreen));
	let mut plot = Plot::new();
	plot.add_trace(trace);
	plot.write_image("norm_histo.png", ImageFormat::PNG, 800, 600, 1.0);
}

pub fn test_scatter_plot() {
	let n: usize = 100;
	let t: Vec<f64> = linspace(0., 10., n).collect();
	let y = t.iter().map(|x| x.sin()).collect();

	let trace = Scatter::new(t, y).mode(Mode::Markers);
	let mut plot = Plot::new();
	plot.add_trace(trace);

	plot.write_image("scatter.png", ImageFormat::PNG, 800, 600, 1.0);
}
