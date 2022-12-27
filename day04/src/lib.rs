type Assignment = (i32, i32);


fn parse_line(line: &str) -> (Assignment, Assignment) {
	let nums = line
		.split(['-', ','])
		.map(str::parse::<i32>)
		.collect::<Result<Vec<_>, _>>()
		.expect("One of the numbers could not be parsed");

	let a1 = (nums[0], nums[1]);
	let a2 = (nums[2], nums[3]);
	(a1, a2)
}


pub fn solve_1(lines: Vec<String>) -> i32 {
	lines
		.iter()
		.map(|s| parse_line(s))
		.filter(|(p1, p2)| (p1.0 >= p2.0 && p1.1 <= p2.1) || (p2.0 >= p1.0 && p2.1 <= p1.1))
		.count()
		.try_into()
		.unwrap()
}


pub fn solve_2(lines: Vec<String>) -> i32 {
	lines
		.iter()
		.map(|s| parse_line(s))
		.filter(|(p1, p2)| (p1.0 >= p2.0 && p1.0 <= p2.1) || (p2.0 >= p1.0 && p2.0 <= p1.1))
		.count()
		.try_into()
		.unwrap()
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let lines = ["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"].map(String::from).to_vec();
		assert_eq!(2, solve_1(lines));
	}

	#[test]
	fn test_2() {
		let lines = ["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"].map(String::from).to_vec();
		assert_eq!(4, solve_2(lines));
	}
}
