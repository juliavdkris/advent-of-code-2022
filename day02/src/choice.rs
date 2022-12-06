use crate::MatchResult;


#[derive(PartialEq, Copy, Clone)]
pub(crate) enum Choice { Rock, Paper, Scissors }

impl Choice {
	pub fn parse_char(c: char) -> Self {
		match c {
			'A' | 'X' => Choice::Rock,
			'B' | 'Y' => Choice::Paper,
			'C' | 'Z' => Choice::Scissors,
			_ => panic!("Invalid choice!")
		}
	}

	pub fn points(&self) -> i32 {
		match self {
			Choice::Rock => 1,
			Choice::Paper => 2,
			Choice::Scissors => 3
		}
	}

	pub fn winning_move(&self) -> Self {
		let mut cycle = [Choice::Rock, Choice::Paper, Choice::Scissors].iter().cycle();

		// Consume iterator until we find player move, next move will be the one that beats player move
		cycle.position(|c| c == self);
		*cycle.next().unwrap()
	}

	pub fn losing_move(&self) -> Self {
		let mut cycle = [Choice::Rock, Choice::Paper, Choice::Scissors].iter().cycle();

		// Consume iterator until we find player move, next next move will be the one that loses to player move
		cycle.position(|c| c == self);
		cycle.next();
		*cycle.next().unwrap()
	}

	pub fn match_against(&self, opponent: &Self) -> MatchResult {
		if self == opponent {
			return MatchResult::Draw;
		}

		let opponent_winning_move = self.winning_move();
		match opponent == &opponent_winning_move {
			true => MatchResult::Loss,
			false => MatchResult::Win
		}
	}
}
