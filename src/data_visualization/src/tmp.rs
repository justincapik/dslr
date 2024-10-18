use plotly::color::NamedColor;
use plotly::common::{Anchor, Marker};
use plotly::Histogram;
use polars::prelude::*;

use plotly::layout::{Annotation, GridPattern, ItemSizing, Layout, LayoutGrid, Legend};
use plotly::{ImageFormat, Plot};

use std::error::Error;

fn make_trace(
	i: usize,
	data: &Series,
	house_name: &str,
) -> Result<Box<Histogram<f64>>, Box<dyn Error>> {
	let mut col: Vec<f64> = data.f64()?.into_iter().flatten().collect();

	col.retain(|x| *x != 0.0);
	if col.is_empty() {
		Err("No float values in column")?;
	}

	let color = match house_name {
		"Gryffindor" => NamedColor::Red,
		"Ravenclaw" => NamedColor::Blue,
		"Slytherin" => NamedColor::Green,
		"Hufflepuff" => NamedColor::Orange,
		_ => NamedColor::Black,
	};

	Ok(Histogram::new(col)
		.marker(Marker::new().color(color))
		.x_axis(format!("x{}", i))
		.y_axis(format!("y{}", i)))
}


pub fn histogram_plot(data: &DataFrame, path_name: &String) -> Result<(), Box<dyn Error>> {
	let mut plot = Plot::new();

	let layoutgrid = LayoutGrid::new()
		.rows(4)
		.columns(4)
		.pattern(GridPattern::Independent);
	let legend = Legend::new()
		.title("Houses")
		.item_sizing(ItemSizing::Constant)
		.border_color(NamedColor::Black)
		.border_width(1)
		.background_color(NamedColor::GhostWhite);
	let mut layout = Layout::new()
		.grid(layoutgrid)
		.legend(legend)
		.title("Hogwarts Houses histogram plots, for each class");

	let mut legend_check = 0;
	let mut i = 1;
	for name in data.get_column_names() {
		// println!("Adding {name}");
		let mut check = false;
		for house_name in ["Ravenclaw", "Slytherin", "Hufflepuff", "Gryffindor"] {
			let one_house: DataFrame = data
				.clone()
				.lazy()
				.filter(col("Hogwarts House").eq(lit(house_name)))
				.collect()?;
			write_trace(
				name,
				house_name,
				i,
				one_house,
				&mut legend_check,
				&mut plot,
				&mut check,
				&mut layout,
			)
		}
		if check {
			i += 1;
		}
	}

	plot.set_layout(layout);

	plot.write_image(path_name, ImageFormat::PNG, 1200, 1200, 1.0);

	Ok(())
}
