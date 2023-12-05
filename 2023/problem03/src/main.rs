use std::fmt::{Display, Formatter};

struct Number {
	x: i32,
	y: i32,
	x_end: i32,
	y_end: i32,
	value: u32
}

impl Number {
	fn new(x: i32, y: i32, x_end: i32, y_end: i32, value: u32) -> Self {
		Number { x, y, x_end, y_end, value}
	}

	fn symbol_nearby(&self, input: &[&str]) -> (bool, char) {
		for i in self.y-1..=self.y_end+1 {
			if i >= 0 && (i as usize) < input.len() - 1 {
				let chars_vec: Vec<char> = input[i as usize].chars().collect();
				for j in self.x-1..=self.x_end+1 {
					if j >= 0 && j < (input[i as usize].len() - 1 as usize) as i32 {
						if !chars_vec[j as usize].is_digit(10)
							&& chars_vec[j as usize] != '.' {
								return (true, chars_vec[j as usize]);
							}
					} 
				}
			}
		}
		return (false, '.');
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "({} {}) -> ({} {}): {}", self.x, self.y, self.x_end, self.y_end, self.value).expect("Couldn't write to the output stream");
		Ok(())
	}
}

fn generate_numbers(input: &[&str]) -> Vec<Number> {
	let mut result: Vec<Number> = Vec::new();
	let mut reading_number: bool = false;
	let mut j: i32;
	let mut j_to: i32 = 0;
	let mut accum_number_chars: String = String::from("");
	for i in 0..input.len() as i32 {
		j = 0;
		for c in input[i as usize].chars()  {
			if !reading_number {
				j_to = j;
			}
			if c.is_digit(10) {
				reading_number = true;
				accum_number_chars.push(c);
				j_to += 1;
			} else {
				if reading_number {
					let new_number = Number::new(j, i, j_to-1, i, accum_number_chars.parse().unwrap());
					println!("new_number = {}", &new_number);
					result.push(new_number);
					j = j_to;
				}
				reading_number = false;
				accum_number_chars = String::from("");
			}
			if !reading_number {
				j += 1;
			}
		}
	}
	return result;
}

fn p1(input: &[&str]) -> u32 {
	let mut result = 0;
	let pieces = generate_numbers(input);
	for piece in pieces.into_iter() {
		let rbool = piece.symbol_nearby(input);
		println!("found: {}: for: {}", &rbool.1, piece);
		if rbool.0 {
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
