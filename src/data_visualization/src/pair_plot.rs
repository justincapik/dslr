use plotly::color::NamedColor;

use plotly::layout::{Annotation, GridPattern, ItemSizing, Layout, LayoutGrid, Legend};
use plotly::{Histogram, Scatter};
use plotly::{ImageFormat, Plot};

use plotly::common::{Anchor, Marker, Mode};

use polars::prelude::*;
use std::error::Error;

fn make_histogram_trace(
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

fn make_scatter_trace(
	i: usize,
	mdata: &Series,
	sdata: &Series,
	house_name: &str,
) -> Result<Box<Scatter<f64, f64>>, Box<dyn Error>> {
	//
	let mut mcol: Vec<f64> = mdata
		.f64()?
		.into_iter()
		.filter(|x| x.is_some())
		.map(|x| x.unwrap())
		.collect();
	let mut scol: Vec<f64> = sdata
		.f64()?
		.into_iter()
		.filter(|x| x.is_some())
		.map(|x| x.unwrap())
		.collect();

	mcol.retain(|x| *x != 0.0);
	scol.retain(|x| *x != 0.0);
	if mcol.is_empty() || scol.is_empty() {
		Err("No float values in column")?;
	}

	let color = match house_name {
		"Gryffindor" => NamedColor::Red,
		"Ravenclaw" => NamedColor::Blue,
		"Slytherin" => NamedColor::Green,
		"Hufflepuff" => NamedColor::Orange,
		_ => NamedColor::Black,
	};

	Ok(Scatter::new(mcol, scol)
		.mode(Mode::Markers)
		.marker(Marker::new().color(color).size(3))
		.x_axis(format!("x{}", i))
		.y_axis(format!("y{}", i)))
}

fn write_histogram_trace(
	main_name: &str,
	house_name: &str,
	i: usize,
	one_house: DataFrame,
	legend_check: &mut usize,
	plot: &mut Plot,
	check: &mut bool,
	layout: &mut Layout,
) {
	let mseries = one_house.column(main_name).unwrap();
	let annot = Annotation::new()
		.y(1.1)
		.x(0.5)
		.y_ref(format!("y{} domain", i))
		.x_ref(format!("x{} domain", i))
		.y_anchor(Anchor::Top)
		.x_anchor(Anchor::Center)
		.show_arrow(false)
		.text(format!("{}", main_name));
	match make_histogram_trace(i, mseries, house_name) {
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
		}
	};
}

fn write_scatter_trace(
	main_name: &str,
	sec_name: &str,
	house_name: &str,
	i: usize,
	one_house: DataFrame,
	legend_check: &mut usize,
	plot: &mut Plot,
	check: &mut bool,
	layout: &mut Layout,
) {
	let mseries = one_house.column(main_name).unwrap();
	let sseries = one_house.column(sec_name).unwrap();
	let annot = Annotation::new()
		.y(1.1)
		.x(0.5)
		.y_ref(format!("y{} domain", i))
		.x_ref(format!("x{} domain", i))
		.y_anchor(Anchor::Top)
		.x_anchor(Anchor::Center)
		.show_arrow(false)
		.text(format!("{} vs {}", main_name, sec_name));

	match make_scatter_trace(i, mseries, sseries, house_name) {
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
		}
	};
}

fn create_single_plot(
	main_name: &str,
	sec_name: &str,
	data: DataFrame,
	i: &mut usize,
	legend_check: &mut usize,
	layout: &mut Layout,
	plot: &mut Plot,
) -> Result<(), Box<dyn Error>> {
	let mut check = false;
	for house_name in ["Ravenclaw", "Slytherin", "Hufflepuff", "Gryffindor"] {
		let one_house: DataFrame = data
			.clone()
			.lazy()
			.filter(col("Hogwarts House").eq(lit(house_name)))
			.collect()?;
		if main_name == sec_name {
			write_histogram_trace(
				main_name,
				house_name,
				*i,
				one_house,
				legend_check,
				plot,
				&mut check,
				layout,
			)
		} else {
			write_scatter_trace(
				main_name,
				sec_name,
				house_name,
				*i,
				one_house,
				legend_check,
				plot,
				&mut check,
				layout,
			)
		}
	}
	if check {
		*i += 1;
	}
	Ok(())
}

pub fn pair_plot(data: &DataFrame, path_name: &String) -> Result<(), Box<dyn Error>> {
	let mut plot = Plot::new();
	let useful_subjects = [
		"Divination",
		"Ancient Runes",
		"Potions",
		"History of Magic",
		"Charms",
		"Flying",
	];

	let layoutgrid = LayoutGrid::new()
		.rows(useful_subjects.len())
		.columns(useful_subjects.len())
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
		.title("Hogwarts Houses pair plot analysis, with relevant classes");

	let mut legend_check = 0;
	let mut i = 1;
	for main_name in useful_subjects {
		for sec_name in useful_subjects {
			create_single_plot(
				main_name,
				sec_name,
				data.clone(),
				&mut i,
				&mut legend_check,
				&mut layout,
				&mut plot,
			)?
		}
		// println!("{main_name} added");
	}
	plot.set_layout(layout);

	plot.write_image(path_name, ImageFormat::PNG, 1500, 1500, 1.0);

	Ok(())
}
