use std::collections::HashSet;


fn sum_priorities(chars: &[char]) -> i32 {
	let alphabet: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
	let mut total = 0;
	for c in chars {
		total += alphabet.iter().position(|x| x == c).unwrap() + 1;
	}
	total.try_into().unwrap()
}


fn common_chars(a: &str, b: &str) -> Vec<char> {
	let a: HashSet<char> = a.chars().collect();
	let b: HashSet<char> = b.chars().collect();

	a.intersection(&b).cloned().collect()
}


fn split_line(line: &str) -> (&str, &str) {
	let first_half = &line[..line.len()/2];
	let second_half = &line[line.len()/2..];

	(first_half, second_half)
}


pub fn solve(lines: Vec<String>) -> i32 {
	let mut chars: Vec<char> = Vec::new();

	for line in lines {
		let (first_half, second_half) = split_line(&line);
		chars.extend(&common_chars(first_half, second_half));
	}

	sum_priorities(&chars)
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse() {
		let line = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");
		let (first_half, second_half) = split_line(&line);
		assert_eq!("vJrwpWtwJgWr", first_half);
		assert_eq!("hcsFMMfFFhFp", second_half);
	}

	#[test]
	fn test() {
		let lines = ["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"].map(String::from).to_vec();
		assert_eq!(157, solve(lines));
	}
}