use polars::prelude::*;
use tabled::{
	settings::{
		object::{Cell, Columns, Rows},
		style::BorderColor,
		themes::Colorization,
		Alignment, Color, Format, Style,
	},
	Table,
};

use crate::Args;

const EMPTY_CELL: &str = "N/A";

pub fn present(table: (Table, Vec<DataType>), _args: &Args) -> PolarsResult<()> {
	let (mut table, types) = table;

	table
		.with(Style::rounded())
		.with(BorderColor::filled(Color::new("\u{1b}[2;35m", "\u{1b}[0m")))
		.modify(Rows::new(1..), Alignment::right());

	for (i, data_type) in types.iter().enumerate() {
		if !data_type.is_numeric() {
			let target = Rows::single(1 + i);

			table
				.with(Colorization::exact(
					[Color::new("\u{1b}[2;3m", "\u{1b}[0m")],
					target,
				))
				.modify(
					target,
					Format::content(|s| {
						if s.is_empty() {
							EMPTY_CELL.to_string()
						} else {
							s.to_string()
						}
					}),
				)
				.modify(target, Alignment::center());
		}

		let target = (1 + i, 1);

		table.modify(Cell::from(target), Alignment::center());

		table.with(Colorization::exact(
			[type_color(data_type)],
			Cell::from(target),
		));
	}

	table
		.modify(Columns::first(), Alignment::left())
		.modify(Columns::single(1), Alignment::center())
		.modify(Rows::first(), Alignment::center())
		.with(Colorization::exact(
			[Color::new("\u{1b}[3m", "\u{1b}[0m")],
			Columns::first(),
		))
		.with(Colorization::exact(
			[Color::new("\u{1b}[1m", "\u{1b}[0m")],
			Rows::first(),
		));

	println!("{}", table.to_string());

	Ok(())
}

fn type_color(data_type: &DataType) -> Color {
	if data_type.is_float() {
		return Color::new("\u{1b}[1;36m", "\u{1b}[0m");
	} else if data_type.is_integer() {
		return Color::new("\u{1b}[1;34m", "\u{1b}[0m");
	} else if data_type.is_string() {
		return Color::new("\u{1b}[1;32m", "\u{1b}[0m");
	} else {
		return Color::new("\u{1b}[1;2;3m", "\u{1b}[0m");
	}
}
