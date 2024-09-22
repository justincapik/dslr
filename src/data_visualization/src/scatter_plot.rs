use plotly::common::{Anchor, Marker, Mode};
use plotly::layout::{Annotation, Layout, Legend};
use plotly::{ImageFormat, Plot, Scatter};

use plotly::color::NamedColor;

use plotly::layout::{GridPattern, ItemSizing, LayoutGrid};

use polars::prelude::*;

use std::error::Error;

fn make_trace(
	i: usize,
	data: &Series,
	house_name: &str,
) -> Result<Box<Scatter<f64, f64>>, Box<dyn Error>> {
	let mut col: Vec<f64> = data
		.f64()?
		.into_iter()
		.filter(|x| x.is_some())
		.map(|x| x.unwrap())
		.collect();

	col.retain(|x| *x != 0.0);
	if col.is_empty() {
		Err("No float values in column")?;
	}

	let t: Vec<f64> = col.iter().copied().rev().collect();
	let y = col;

	let color = match house_name {
		"Gryffindor" => NamedColor::Red,
		"Ravenclaw" => NamedColor::Blue,
		"Slytherin" => NamedColor::Green,
		"Hufflepuff" => NamedColor::Orange,
		_ => NamedColor::Black,
	};

	Ok(Scatter::new(t, y)
		.mode(Mode::Markers)
		.marker(Marker::new().color(color).size(3))
		.x_axis(format!("x{}", i))
		.y_axis(format!("y{}", i)))
}

fn write_trace(
	name: &str,
	house_name: &str,
	i: usize,
	one_house: DataFrame,
	legend_check: &mut usize,
	plot: &mut Plot,
	check: &mut bool,
	layout: &mut Layout,
) {
	let series = one_house.column(name).unwrap();
	let annot = Annotation::new()
		.y(1.1)
		.x(0.5)
		.y_ref(format!("y{} domain", i))
		.x_ref(format!("x{} domain", i))
		.y_anchor(Anchor::Top)
		.x_anchor(Anchor::Center)
		.show_arrow(false)
		.text(name);

	match make_trace(i, series, house_name) {
		Err(_) => (),
		Ok(mut trace) => {
			if *legend_check < 4 {
				trace = trace.name(house_name);
				*legend_check += 1;
			} else {
				trace = trace.show_legend(false);
			}
			*check = true;
			plot.add_trace(trace);
			(*layout).add_annotation(annot);
			// println!("{name} grade for {house_name} added !");
		}
	};
}

pub fn simple_scatter_plot(data: &DataFrame, path_name: &String) -> Result<(), Box<dyn Error>> {
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
		.title("Hogwarts Houses scatter plots, for each class");

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
