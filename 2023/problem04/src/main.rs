use regex::Regex;
use std::env;
use std::collections::HashMap;

// {{{ p1(input: &[&str]) -> u64
fn p1(input: &[&str]) -> u64 {
	let mut result = 0;
	let reg: Regex = Regex::new(r"Card\s+(?<card>\d+):(?<winners>(\s+\d+)+)\s*\|\s*(?<mycards>(\s+\d+)+)").unwrap();
	for s in input {
		let mut internal_count: u64 = 0;
		if let Some(matches) = reg.captures(s){
			let card: u64 = matches["card"].trim().parse().expect("No card available");
			let winners: Vec<u64> = matches["winners"]
				.split(' ')
				.filter(|d| d != &"")
				.map(|k| k.trim().parse::<u64>().unwrap())
				.collect();
			matches["mycards"]
				.split(' ')
			.filter(|d| d != &"")
			.for_each(|val| {
				match val.trim().parse::<u64>() {
					Ok(v) => {internal_count += if winners.contains(&v) {1} else {0}},
					Err(_) => {}
				}
			});
			if internal_count > 0 {
				result += 1*2u64.pow(internal_count as u32 - 1 );
			}
		}
	}
	return result;
}
// }}}

// {{{ p2(input: &[&str]) -> u64 
fn p2(input: &[&str]) -> u64 {
	let mut result = 0;
	let mut v: Vec<u64> = vec![1u64];
	let reg: Regex = Regex::new(r"Card\s+(?<card>\d+):(?<winners>(\s+\d+)+)\s*\|\s*(?<mycards>(\s+\d+)+)").unwrap();
	for s in input {
		let mut internal_count: u64 = 0;
		if let Some(matches) = reg.captures(s){
			let card: u64 = matches["card"].trim().parse().expect("No card available");
			let winners: Vec<u64> = matches["winners"]
				.split(' ')
				.filter(|d| d != &"")
				.map(|k| k.trim().parse::<u64>().unwrap())
				.collect();
			matches["mycards"]
				.split(' ')
				.filter(|d| d != &"")
				.for_each(|val| {match val.trim().parse::<u64>() {
						Ok(v) => {internal_count += if winners.contains(&v) {1} else {0}},
						Err(_) => {}
					}
				});
			if v.len() < (card + internal_count) as usize {
				v.resize((card + internal_count) as usize, 1);
			}
			for i in (card) as usize..(card + internal_count) as usize {
				v[i] += v[(card-1) as usize];
			}
		}
	}
	for val in &v {
		result += val;
	}
	
	return result;
}
// }}}


fn main() {
	env::set_var("RUST_BACKTRACE", "1");
	let raw_input = std::fs::read_to_string("./src/input.txt").expect("Couldn't read file");
	let processed_input: Vec<&str> = raw_input.split('\n').collect();
	let r1 = p1(&processed_input);
	let r2 = p2(&processed_input);
	println!("P1: {}\nP2: {}", r1, r2);
}


#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_working() {
		let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
		let ex: Vec<&str> = test_input.split('\n').collect();
		assert_eq!(p1(&ex), 13);
	}
	#[test]
	fn p2_working() {
		let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
		let ex: Vec<&str> = test_input.split('\n').collect();
		assert_eq!(p2(&ex), 30);
	}
}
