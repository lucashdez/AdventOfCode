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
	return result;
}

fn main() {
	let path: &str = "./src/input.txt";
	let input = std::fs::read_to_string(path)
		.expect(format!("Could not read the input file. Not existent or corrupt in {}", path).as_str());
	let r1: u32 = p1(&input.split("\n").collect::<Vec<&str>>());
	let r2: u32 = 0;

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
