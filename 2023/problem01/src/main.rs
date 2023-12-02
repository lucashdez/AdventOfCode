use std::collections::HashMap;

fn p1(input: &[&str]) -> u32 {
	let mut result: u32 = 0;
	input.into_iter().for_each(|s| {
		if let Some(f) = s.chars().find(|x| x.is_digit(10)) {
			// SAFETY: If there is one, there is infinite
			let b = s.chars().rfind(|x| x.is_digit(10)).unwrap();
			let number: u32 = format!("{}{}",f,b).as_str().parse().unwrap();
			result += number;
		}
	});
	return result;
}

fn p2(input: &[&str]) -> u32 {
	let mut result = 0;
	let mut numbers_hash: HashMap<&str,&str> = HashMap::new();
	numbers_hash.insert("one", "1");
	numbers_hash.insert("two", "2");
	numbers_hash.insert("three", "3");
	numbers_hash.insert("four", "4");
	numbers_hash.insert("five", "5");
	numbers_hash.insert("six", "6");
	numbers_hash.insert("seven", "7");
	numbers_hash.insert("eight", "8");
	numbers_hash.insert("nine", "9");
	numbers_hash.insert("1", "1");
	numbers_hash.insert("2", "2");
	numbers_hash.insert("3", "3");
	numbers_hash.insert("4", "4");
	numbers_hash.insert("5", "5");
	numbers_hash.insert("6", "6");
	numbers_hash.insert("7", "7");
	numbers_hash.insert("8", "8");
	numbers_hash.insert("9", "9");
	input.into_iter().for_each(|s| {
		let mut f:(&str, usize) = ("", 99999);
		let mut b:(&str, usize) = ("", 0);
		numbers_hash
			.clone()
			.into_iter()
			.for_each(|(k, _)| {
				let mut all_idx: Vec<(usize, &str)> = s.match_indices(k).collect::<Vec<(usize, &str)>>();
				if !all_idx.is_empty() {
					all_idx.sort_by(|a,b| {a.0.cmp(&b.0)});
					f = if all_idx[0].0 <= f.1 {(all_idx[0].1, all_idx[0].0)} else {f};
					b = if all_idx[all_idx.len()-1].0 >= b.1 {(all_idx[all_idx.len()-1].1, all_idx[all_idx.len()-1].0)} else {b};
				}
			});
		if f.0 != "" {
			let number: u32 = format!("{}{}",numbers_hash.get(f.0).unwrap(),numbers_hash.get(b.0).unwrap()).as_str().parse().unwrap();
			result += number;
		}
	});
	return result;
}

fn main() {
	let path: &str = "./src/input.txt";
	let input = std::fs::read_to_string(path)
		.expect(format!("Could not read the input file. Not existent or corrupt in {}", path).as_str());
	let r1: u32 = p1(&input.split("\n").collect::<Vec<&str>>());
	let r2: u32 = p2(&input.split("\n").collect::<Vec<&str>>());

	print!("P1: {}\nP2: {}", r1, r2);
}

#[cfg(test)]
mod tests {
	use super::{p1, p2};

	#[test]
	fn it_works_with_p1() {
		let test_input_str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"
			.split('\n')
			.collect::<Vec<&str>>();
		assert_eq!(p1(&test_input_str), 142);
	}

	#[test]
	fn it_works_with_p2() {
		let test_input_str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"
			.split('\n')
			.collect::<Vec<&str>>();
		assert_eq!(p2(&test_input_str), 281);
	}
}
