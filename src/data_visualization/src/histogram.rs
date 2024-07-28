#![allow(dead_code)]
#![allow(unused)]

use itertools_num::linspace;
use plotly::common::{
	self, ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{self, Annotation, Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::Scatter;
use plotly::{ImageFormat, Plot};

use plotly::box_plot::{BoxMean, BoxPoints};
use plotly::color::{NamedColor, Rgb, Rgba};
use plotly::common::{Anchor, ErrorData, ErrorType, Orientation};
use plotly::histogram::{Bins, Cumulative, HistFunc, HistNorm};
use plotly::layout::{BoxMode, Margin, TraceOrder};
use plotly::{Bar, BoxPlot, Histogram};
use polars::error::{polars_bail, PolarsError};
use polars::frame::DataFrame;
use polars::prelude::{col, lit, IntoLazy};
use polars::series::Series;
use rand;
use rand_distr::{Distribution, Normal, Uniform};

use plotly::common::Side;
use plotly::layout::ItemSizing;
use plotly::layout::{GridPattern, LayoutGrid, RowOrder};

use std::{error::Error, process};

fn make_trace(
	name: &str,
	i: usize,
	data: &Series,
	house_name: &str,
) -> Result<Box<Histogram<f64>>, Box<dyn Error>> {
	//
	let mut col: Vec<f64> = data
		.f64()?
		.into_iter()
		.filter(|x| x.is_some())
		.map(|x| x.unwrap())
		.collect();

	col.retain(|x| *x != 0.0);
	let slice: Vec<&f64> = col.iter().take(3).collect();
	println!("{name}: {:?}...", slice);
	if (col.is_empty()) {
		Err("No float values in column")?;
	}

	let min = 0.0;
	let max = col.len() as f64;
	let n = (max - min) as usize;
	// col.sort_by(|a, b| a.partial_cmp(b).unwrap());
	//let t = sample_uniform_distribution(n, min, max);
	let y = col;

	let color = match house_name {
		"Gryffindor" => NamedColor::Red,
		"Ravenclaw" => NamedColor::Blue,
		"Slytherin" => NamedColor::Green,
		"Hufflepuff" => NamedColor::Orange,
		_ => NamedColor::Black,
	};

	Ok(Histogram::new(y)
		.marker(Marker::new().color(color))
		.x_axis(format!("x{}", i))
		.y_axis(format!("y{}", i)))
}

pub fn histogram_plot(data: DataFrame) -> Result<(), Box<dyn Error>> {
	let mut plot = Plot::new();

	let mut layout = Layout::new()
		.grid(
			LayoutGrid::new()
				.rows(4)
				.columns(4)
				.pattern(GridPattern::Independent),
		)
		.legend(
			Legend::new()
				.title("Houses")
				.item_sizing(ItemSizing::Constant)
				.border_color(NamedColor::Black)
				.border_width(1)
				.background_color(NamedColor::GhostWhite),
		)
		.title("Hogwarts Houses histograms, for each class");

	let mut legend_check = 0;
	let mut i = 1;
	for name in data.get_column_names() {
		println!("Adding {name}");
		let mut check = false;
		for house_name in ["Ravenclaw", "Slytherin", "Hufflepuff", "Gryffindor"] {
			let one_house: DataFrame = data
				.clone()
				.lazy()
				.filter(col("Hogwarts House").eq(lit(house_name)))
				.collect()?;
			let series = one_house.column(name).unwrap();
			match make_trace(name, i, series, house_name) {
				Err(e) => println!("column error: {}", e),
				Ok(mut trace) => {
					if (legend_check < 4) {
						trace = trace.name(house_name);
						legend_check += 1;
					} else {
						trace = trace.show_legend(false);
					}
					check = true;
					plot.add_trace(trace);
					layout.add_annotation(
						Annotation::new()
							.y(1.1)
							.x(0.5)
							.y_ref(format!("y{} domain", i))
							.x_ref(format!("x{} domain", i))
							.y_anchor(Anchor::Top)
							.x_anchor(Anchor::Center)
							.show_arrow(false)
							.text(format!("{}", name)),
					);
					println!("{name} added !");
				}
			};
		}
		if (check) {
			i += 1;
		}
	}

	plot.set_layout(layout);

	plot.write_image("histogram.png", ImageFormat::PNG, 1200, 1200, 1.0);

	Ok(())
}
