#[derive(PartialEq, Debug)]
enum MatchResult { Win, Draw, Loss }

impl MatchResult {
	fn parse_char(c: char) -> Self {
		match c {
			'X' => MatchResult::Loss,
			'Y' => MatchResult::Draw,
			'Z' => MatchResult::Win,
			_ => panic!("Invalaid match result!")
		}
	}

	fn points(&self) -> i32 {
		match self {
			MatchResult::Win => 6,
			MatchResult::Draw => 3,
			MatchResult::Loss => 0
		}
	}
}


#[derive(PartialEq, Copy, Clone)]
enum Choice { Rock, Paper, Scissors }

impl Choice {
	fn parse_char(c: char) -> Self {
		match c {
			'A' | 'X' => Choice::Rock,
			'B' | 'Y' => Choice::Paper,
			'C' | 'Z' => Choice::Scissors,
			_ => panic!("Invalid choice!")
		}
	}

	fn points(&self) -> i32 {
		match self {
			Choice::Rock => 1,
			Choice::Paper => 2,
			Choice::Scissors => 3
		}
	}

	fn winning_move(&self) -> Self {
		let mut cycle = [Choice::Rock, Choice::Paper, Choice::Scissors].iter().cycle();

		// Consume iterator until we find player move, next move will be the one that beats player move
		cycle.position(|c| c == self);
		*cycle.next().unwrap()
	}

	fn losing_move(&self) -> Self {
		let mut cycle = [Choice::Rock, Choice::Paper, Choice::Scissors].iter().cycle();

		// Consume iterator until we find player move, next move will be the one that beats player move
		cycle.position(|c| c == self);
		cycle.next();
		*cycle.next().unwrap()
	}

	fn match_against(&self, opponent: &Self) -> MatchResult {
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


fn parse_line(line: String) -> (Choice, Choice) {
	let mut chars = line.chars();
	let opponent = Choice::parse_char(chars.next().unwrap());
	chars.next();
	let player = Choice::parse_char(chars.next().unwrap());
	(opponent, player)
}


fn parse_line2(line: String) -> (Choice, MatchResult) {
	let mut chars = line.chars();
	let opponent = Choice::parse_char(chars.next().unwrap());
	chars.next();
	let match_result = MatchResult::parse_char(chars.next().unwrap());
	(opponent, match_result)
}


pub fn solve_1(lines: Vec<String>) -> i32 {
	let mut points = 0;

	for line in lines {
		let (opponent, player) = parse_line(line);
		points += player.points() + player.match_against(&opponent).points();
	}

	points
}

pub fn solve_2(lines: Vec<String>) -> i32 {
	let mut points = 0;

	for line in lines {
		let (opponent, match_result) = parse_line2(line);
		let player = match match_result {
			MatchResult::Win => opponent.winning_move(),
			MatchResult::Draw => opponent,
			MatchResult::Loss => opponent.losing_move()
		};
		points += player.points() + player.match_against(&opponent).points();
	}

	points
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_match_win() {
		let player = Choice::Paper;
		let opponent = Choice::Rock;
		assert_eq!(MatchResult::Win, player.match_against(&opponent));
	}

	#[test]
	fn test_match_lose() {
		let player = Choice::Rock;
		let opponent = Choice::Paper;
		assert_eq!(MatchResult::Loss, player.match_against(&opponent));
	}

	#[test]
	fn test_example_input() {
		let lines = ["A Y", "B X", "C Z"].map(String::from).to_vec();
		assert_eq!(15, solve_1(lines));
	}
}
