mod lib;
use lib::hand::*;

fn p1(s: &str) -> u64 {
	let mut hands: Vec<Hand> = Vec::new();
	s.split('\n')
		.filter(|f| f != &"")
		.into_iter()
		.for_each(|c| {
			let a: Vec<&str> = c.split(" ").filter(|f| f != &"").collect();
			if a.len() == 2 {
				hands.push(Hand::new(a[0], a[1].parse().unwrap()))
			}
		});
	let hands_len = hands.len();
	for i in 0..hands_len {
		hands[i].check_hand();
	}
	hands.sort();
	let max_len = hands.len();
	let mut result: u64 = 0; 
	for i in 0..max_len {
		result += ((i+1) * hands[i].get_bid()) as u64;
	}
	return result;
}

fn p2(s: &str) -> u64 {
	let mut hands: Vec<Hand> = Vec::new();
	s.split('\n')
		.filter(|f| f != &"")
		.into_iter()
		.for_each(|c| {
			let a: Vec<&str> = c.split(" ").filter(|f| f != &"").collect();
			if a.len() == 2 {
				hands.push(Hand::new_p2(a[0], a[1].parse().unwrap()))
			}
		});
	let hands_len = hands.len();
	for i in 0..hands_len {
		hands[i].check_hand_p2();
	}
	hands.sort();
	dbg!(&hands);
	let max_len = hands.len();
	let mut result: u64 = 0; 
	for i in 0..max_len {
		result += ((i+1) * hands[i].get_bid()) as u64;
	}
	return result;
}

fn main() {
	let str = std::fs::read_to_string("./src/input.txt")
		.expect("Cannot read. Bad luck.");
	let r1 = p1(str.as_str());
	let r2 = p2(str.as_str());
	println!("P1: {}\nP2: {}", r1, r2);

}


#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_working() {
		let str = std::fs::read_to_string("./src/ex.txt")
			.expect("Cannot read. Bad luck.");
		assert_eq!(p1(str.as_str()), 6440);
	}
	#[test]
	fn p1_test() {
		let str = std::fs::read_to_string("./src/input.txt")
			.expect("bad luck");
		assert_eq!(p1(str.as_str()), 253933213);
	}

	#[test]
	fn p2_working() {
		let str = std::fs::read_to_string("./src/ex.txt")
			.expect("bad luck");
		assert_eq!(p2(str.as_str()), 5905);
	}
}
