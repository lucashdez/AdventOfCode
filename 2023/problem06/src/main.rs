mod lib;
use lib::parser::*;

fn p2(input: &(i64, i64)) -> i64 {
	let mut result: i64 = 0;
	let max_time = input.0;
	let objective = input.1;
	for i in 0..max_time {
		let possible = (max_time - i) * i;
		if possible > objective {
			result += 1;
		}
	} 
	result
}

fn p1(input: &Vec<(i64, i64)>) -> i64 {
	let mut result: i64 = 1;
	for race in input {
		let max_time = race.0;
		let objective = race.1;
		let mut ways = 0;
		for i in 0..max_time {
			let possible = (max_time - i) * i;
			if possible > objective {
				ways += 1;
			}
		} 
		result *= ways;
	}
	result 
}

fn main() {
	let lect = std::fs::read_to_string("./src/input.txt")
		.expect("Couldn't read file");
	let parser: Parser = Parser::new(0);
	let t = parser.parse(lect.as_str());
	let r1 = p1(&t);
	let t2 = parser.parse2(lect.as_str());
	let r2 = p2(&t2);
	println!("P1: {}\nP2: {}", r1, r2);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn p1_working() {
		let lect = std::fs::read_to_string("./src/ex.txt")
			.expect("Couldn't read the file");
		let p = Parser::new(0);
		let t = p.parse(lect.as_str());
		assert_eq!(p1(&t), 288)
	}

	#[test]
	fn p2_working() {
		let lect = std::fs::read_to_string("./src/ex.txt")
			.expect("Couldn't read the file");
		let p = Parser::new(0);
		let t = p.parse2(lect.as_str());
		assert_eq!(p2(&t), 71503)
	}
}
