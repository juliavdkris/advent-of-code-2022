mod choice;
mod match_result;

use choice::Choice;
use match_result::MatchResult;


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
