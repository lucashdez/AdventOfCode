use std::fmt::{Display, Formatter};

struct Number {
	x: u32,
	y: u32,
	x_end: u32,
	y_end: u32,
	value: u32
}

impl Number {
	fn new(x: u32, y: u32, x_end: u32, y_end: u32, value: u32) -> Self {
		Number { x, y, x_end, y_end, value}
	}

	fn symbol_nearby(&self, input: &[&str]) -> bool {
		let result: bool = false;
		return result;
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.value).expect("Couldn't write to the output stream");
		Ok(())
	}
}

fn generate_numbers(input: &[&str]) -> Vec<Number> {
	let mut result: Vec<Number> = Vec::new();
	let mut reading_number: bool = false;
	let mut j: u32;
	let mut j_to: u32 = 0;
	let mut accum_number_chars: String = String::from("");
	for i in 0..input.len() as u32 {
		j = 0;
		for c in input[0].chars()  {
			if !reading_number {
				j_to = j;
			}
			if c.is_digit(10) {
				reading_number = true;
				accum_number_chars.push(c);
				j_to += 1;
			} else {
				if reading_number {
					result.push(Number::new(j, i, j_to, i, accum_number_chars.parse().unwrap()));
				}
				reading_number = false;
				accum_number_chars = String::from("");
			}
			j += 1;
		}
	}
	return result;
}

fn p1(input: &[&str]) -> u32 {
	let mut result = 0;
	let pieces = generate_numbers(input);
	for piece in pieces.into_iter() {
		if piece.symbol_nearby(input) {
			result += piece.value;
		}
	}

	return result;
}

fn p2(input: &[&str]) -> u32 {
	return 0;
}

fn main() {
	let input = std::fs::read_to_string("./src/input.txt").expect("Cannot open the file");
	let input_arr: Vec<&str> = input.split('\n').collect(); 
	let r1 = p1(&input_arr);
	let r2 = p2(&input_arr);
    println!("P1: {}\nP2: {}", r1, r2);
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_working() {
		let test_input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
		let ti_arr: Vec<&str> = test_input.split('\n').collect();
		assert_eq!(p1(&ti_arr), 4361);

	}
}
