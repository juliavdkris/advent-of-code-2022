#[derive(PartialEq, Debug)]
pub(crate) enum MatchResult { Win, Draw, Loss }

impl MatchResult {
	pub fn parse_char(c: char) -> Self {
		match c {
			'X' => MatchResult::Loss,
			'Y' => MatchResult::Draw,
			'Z' => MatchResult::Win,
			_ => panic!("Invalaid match result!")
		}
	}

	pub fn points(&self) -> i32 {
		match self {
			MatchResult::Win => 6,
			MatchResult::Draw => 3,
			MatchResult::Loss => 0
		}
	}
}
