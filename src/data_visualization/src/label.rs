use plotly::color::NamedColor;

use super::LABEL_NAME;

pub enum Label {
	Ravenclaw,
	Slytherin,
	Hufflepuff,
	Gryffindor,
}

impl std::fmt::Display for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Label::Ravenclaw => write!(f, "Ravenclaw"),
			Label::Slytherin => write!(f, "Slytherin"),
			Label::Hufflepuff => write!(f, "Hufflepuff"),
			Label::Gryffindor => write!(f, "Gryffindor"),
		}
	}
}

impl std::str::FromStr for Label {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Ravenclaw" => Ok(Label::Ravenclaw),
			"Slytherin" => Ok(Label::Slytherin),
			"Hufflepuff" => Ok(Label::Hufflepuff),
			"Gryffindor" => Ok(Label::Gryffindor),
			_ => Err(format!("{s} is not a valid {LABEL_NAME}")),
		}
	}
}

impl Label {
	pub fn color(&self) -> NamedColor {
		match self {
			Label::Ravenclaw => NamedColor::Blue,
			Label::Slytherin => NamedColor::Green,
			Label::Hufflepuff => NamedColor::Orange,
			Label::Gryffindor => NamedColor::Red,
		}
	}
}
