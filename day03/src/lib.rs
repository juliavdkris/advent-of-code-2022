use std::collections::HashSet;


fn sum_priorities(chars: &[char]) -> i32 {
	let alphabet: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
	let mut total = 0;
	for c in chars {
		total += alphabet.iter().position(|x| x == c).unwrap() + 1;
	}
	total.try_into().unwrap()
}


fn common_chars(strings: &[&str]) -> Vec<char> {
	let mut common: HashSet<char> = strings.first().unwrap().chars().collect();

	for string in &strings[1..] {
		let set: HashSet<char> = string.chars().collect();
		common = common.intersection(&set).copied().collect();
	}

	common.into_iter().collect()
}


pub fn solve_1(lines: Vec<String>) -> i32 {
	let mut chars: Vec<char> = Vec::new();

	for line in lines {
		let first_half = &line[..line.len()/2];
		let second_half = &line[line.len()/2..];
		chars.extend(&common_chars(&[first_half, second_half]));
	}

	sum_priorities(&chars)
}


pub fn solve_2(lines: Vec<String>) -> i32 {
	let mut chars: Vec<char> = Vec::new();

	for chunk in lines.chunks_exact(3) {
		if let [l1, l2, l3] = chunk {
			chars.extend(common_chars(&[l1, l2, l3]));
		}
	}

	sum_priorities(&chars)
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let lines = ["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"].map(String::from).to_vec();
		assert_eq!(157, solve_1(lines));
	}

	#[test]
	fn test_2() {
		let lines = ["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"].map(String::from).to_vec();
		assert_eq!(70, solve_2(lines));
	}
}
