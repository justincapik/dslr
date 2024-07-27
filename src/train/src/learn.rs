use indicatif::ProgressIterator;
use model::Model;

use crate::{group::GroupedRow, Args};

pub fn learn(arg: &Args, grouped_row: GroupedRow) {
	let mut model = Model::default();

	for i in (0..arg.iteration).progress() {
		// sleep 0.1 second
		std::thread::sleep(std::time::Duration::from_millis(10));
	}
}
