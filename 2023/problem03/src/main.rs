use regex::Regex;
use std::fmt::{Display, Formatter};
use std::collections::HashMap;

struct Number {
	x: i64,
	y: i64,
	x_end: i64,
	y_end: i64,
	value: u64
}

impl Number {
	fn new(x: i64, y: i64, x_end: i64, y_end: i64, value: u64) -> Self {
		Number { x, y, x_end, y_end, value}
	}

	fn symbol_nearby(&self, input: &[&str]) -> (bool, char) {
		for i in self.y-1..=self.y_end+1 {
			if i >= 0 && (i as usize) < input.len() {
				let chars_vec: Vec<char> = input[i as usize].chars().collect();
				for j in self.x-1..=self.x_end+1 {
					if j >= 0 && j < (input[i as usize].len() as usize) as i64 {
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
	fn gear_nearby(&self, input: &[&str]) -> (bool, (i64, i64)) {
		for i in self.y-1..=self.y_end+1 {
			if i >= 0 && (i as usize) < input.len() {
				let chars_vec: Vec<char> = input[i as usize].chars().collect();
				for j in self.x-1..=self.x_end+1 {
					if j >= 0 && j < (input[i as usize].len() as usize) as i64 {
						if !chars_vec[j as usize].is_digit(10)
							&& chars_vec[j as usize] == '*' {
								return (true, (j,i));
							}
					} 
				}
			}
		}
		return (false, (0,0));
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
	let reg = Regex::new(r"\d+").unwrap(); 
	for i in 0..input.len() {
		for m in reg.find_iter(input[i]) {
			result.push(
				Number::new(m.start() as i64,
							i as i64,
							(m.end()) as i64 - 1,
							i as i64,
							m.as_str().parse().unwrap())
			);
		}
	}
	
	return result;
}

fn p1(input: &[&str]) -> u64 {
	let mut result = 0;
	let pieces = generate_numbers(input);
	for piece in pieces.into_iter() {
		let rbool = piece.symbol_nearby(input);
		if rbool.0 {
			result += piece.value;
		}
	}

	return result;
}

struct Gear {
	x: i64,
	y: i64,
	parts: Vec<Number>,
}
impl Gear {
	fn new(x: i64, y: i64, val: Number) -> Gear {
		let mut new_vec: Vec<Number> = Vec::new();
		new_vec.push(val);
		Gear{x, y, parts: new_vec}
	}
	fn add(&mut self, val: Number) {
		self.parts.push(val);
	}
}

fn p2(input: &[&str]) -> u64 {
	let mut result = 0;
	let mut gears: HashMap<(i64,i64),Gear> = HashMap::new();
	let pieces = generate_numbers(input);
	for piece in pieces.into_iter() {
		let possible = piece.gear_nearby(input);
		if possible.0 {
			let x = possible.1.0;
			let y = possible.1.1;
			if gears.contains_key(&(x,y)) {
				gears.entry((x,y)).and_modify(|v| v.parts.push(piece));
			} else {
				gears.insert((x,y), Gear::new(x,y,piece));
			}
		}
	}
	for gear in gears.into_values() {
		if gear.parts.len() == 2 {
			result += gear.parts[0].value * gear.parts[1].value;
		}
	}
	return result;
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

	
	#[test]
	fn p1_real() {
	let input = std::fs::read_to_string("./src/input.txt").expect("Cannot open the file");
	let input_arr: Vec<&str> = input.split('\n').collect(); 
		assert_eq!(p1(&input_arr), 526404)
	}

	#[test]
	fn p2_working() {
		let test_input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
		let ti_arr: Vec<&str> = test_input.split('\n').collect();
		assert_eq!(p2(&ti_arr), 467835);
	}
	
	
}
