fn sort_by_calories(lines: Vec<String>) -> Vec<i32> {
	let mut elves: Vec<i32> = vec![0];

	for line in lines {
		if line.is_empty() {
			elves.push(0);
		}
		else {
			let elf = elves.last_mut().expect("No last line");
			let num = line.parse::<i32>().expect("Could not parse line");
			*elf += num;
		}
	}

	elves.sort();
	elves
}


pub fn solve_1(lines: Vec<String>) -> i32 {
	*sort_by_calories(lines).last().unwrap()
}

pub fn solve_2(lines:Vec<String>) -> i32{
	let calories = sort_by_calories(lines);
	calories.iter().rev().take(3).sum()
}


#[cfg(test)]
mod tests {
    use crate::*;

	#[test]
	fn test_1() {
		let lines = ["1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "", "10000"].map(String::from).to_vec();
		assert_eq!(24000, solve_1(lines));
	}

	#[test]
	fn test_2() {
		let lines = ["1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "", "10000"].map(String::from).to_vec();
		assert_eq!(45000, solve_2(lines));
	}
}
