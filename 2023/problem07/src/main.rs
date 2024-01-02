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
	dbg!(&hands);
	hands.into_iter()
		.enumerate()
		.map(|(pos, h)| {
			(h.get_value() * (pos + 1)) as u64
		}).sum()

}

fn main() {
	let str = std::fs::read_to_string("./src/input.txt")
		.expect("Cannot read. Bad luck.");
	let r1 = p1(str.as_str());
	println!("P1: {}\nP2: {}", r1, 0);

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
}
