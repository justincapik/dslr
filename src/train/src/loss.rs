use float::Float;
use hypothesis::one_vs_all;
use model::Model;
use tabled::{
	builder::Builder,
	settings::{
		object::{Cell, Columns},
		style::{BorderColor, BorderSpanCorrection, HorizontalLine},
		themes::Colorization,
		Alignment, Color, Modify, Span, Style, Theme,
	},
	Table,
};

use crate::prepare::{Dataset, GroupedDatasets};

type TableRecord<'s> = [&'s str; 7];

const LABEL: &str = "label";
const TRAINING: &str = "training data";
const TESTING: &str = "test data";

const CORRECT: &str = "✓";
const TOTAL: &str = "#";
const PERCENT: &str = "%";
const HEADERS: TableRecord<'static> = ["", CORRECT, TOTAL, PERCENT, CORRECT, TOTAL, PERCENT];

pub fn print_result(grouped_datasets: &GroupedDatasets, model: &Model) {
	let mut builder = Builder::default();

	builder.push_record([""]);
	builder.push_record(HEADERS);

	let mut table = fill_table(builder, grouped_datasets, model);

	let mut theme = Theme::from_style(Style::rounded());
	theme.remove_horizontal_lines();
	theme.insert_horizontal_line(1, HorizontalLine::inherit(Style::modern()));
	theme.insert_horizontal_line(2, HorizontalLine::inherit(Style::modern()));

	table
		.with(
			Modify::new(Cell::new(0, 0))
				.with(Span::row(2))
				.with(Alignment::center_vertical())
				.with(LABEL),
		)
		.with(
			Modify::new(Cell::new(0, 1))
				.with(Span::column(3))
				.with(TRAINING),
		)
		.with(
			Modify::new(Cell::new(0, 4))
				.with(Span::column(3))
				.with(TESTING),
		)
		.with(theme)
		.with(Alignment::center_vertical())
		.with(Alignment::center())
		.with(Colorization::exact(
			[Color::new("\u{1b}[1;32m", "\u{1b}[0m")],
			Columns::single(1),
		))
		.with(Colorization::exact(
			[Color::new("\u{1b}[1;32m", "\u{1b}[0m")],
			Columns::single(4),
		))
		.with(Colorization::exact(
			[Color::new("\u{1b}[1m", "\u{1b}[0m")],
			Columns::single(2),
		))
		.with(Colorization::exact(
			[Color::new("\u{1b}[1m", "\u{1b}[0m")],
			Columns::single(5),
		))
		.with(Colorization::exact(
			[Color::new("\u{1b}[1;36m", "\u{1b}[0m")],
			Columns::single(3),
		))
		.with(Colorization::exact(
			[Color::new("\u{1b}[1;36m", "\u{1b}[0m")],
			Columns::single(6),
		))
		.with(BorderColor::filled(Color::new("\u{1b}[2;35m", "\u{1b}[0m")))
		.with(BorderSpanCorrection);

	println!("{table}");
}

fn fill_table(mut builder: Builder, grouped_datasets: &GroupedDatasets, model: &Model) -> Table {
	for (label, datasets) in grouped_datasets.iter() {
		let training_correct = dataset_loss(label, &datasets.training, model);
		let training_total = datasets.training.len();
		let training_percent = training_correct as Float / training_total as Float * 100.0;

		let testing_correct = dataset_loss(label, &datasets.testing, model);
		let testing_total = datasets.testing.len();
		let testing_percent = testing_correct as Float / testing_total as Float * 100.0;

		let record: TableRecord = [
			label,
			&training_correct.to_string(),
			&training_total.to_string(),
			&format!("{training_percent:.2}%"),
			&testing_correct.to_string(),
			&testing_total.to_string(),
			&format!("{testing_percent:.2}%"),
		];

		builder.push_record(record);
	}

	builder.build()
}

fn dataset_loss(truth: &str, dataset: &Dataset, model: &Model) -> usize {
	let mut correct = 0;

	for row in dataset {
		let prediction = one_vs_all(&row, model);
		if &prediction == truth {
			correct += 1;
		}
	}

	correct
}
