mod util;

type Day = (u8, fn(Vec<String>) -> i32);
const DAYS: &[Day] = &[
	(1, day01::solve_1),
	(1, day01::solve_2),
	(2, day02::solve_1),
	(2, day02::solve_2),
	(3, day03::solve_1),
	(3, day03::solve_2),
	(4, day04::solve_1),
	(4, day04::solve_2),
];


fn main() {
	for day in DAYS {
		let solution = day.1(util::read_lines(day.0));
		println!("Day {day}: {solution}", day=day.0);
	}
}
