mod lib;
use lib::{
	maps::*,
	parser::*
};
	

/*fn p1(m: RangesMap) -> u64 {
	let mut locations: Vec<u64> = Vec::new();
	let mut seeds = m.seeds.clone(); 
	seeds.into_iter().for_each(|s| {
		locations.push(m.seed_to_location(s));
	}); 
	locations.into_iter().min().unwrap()
}
*/

/*fn p2(m: RangesMap) -> u64 {
	let mut seeds_iters = m.seeds.clone();
	let mut locations: Vec<u64> = Vec::new();
	let mut all_ranges: Vec<std::ops::Range<u64>> = Vec::new();
	let mut i = 0;
	while i < seeds_iters.len() {
		let maxit = seeds_iters[i] + seeds_iters[i+1];
		let minit = seeds_iters[i];
		all_ranges.push(minit..maxit);
		i += 2;
	}

	for r in all_ranges {
		locations.push(m.ranges_to_location(r));
	}
	
	return locations.into_iter().min().unwrap();
}
*/

fn main() {
	let s = std::fs::read_to_string("./src/input.txt").expect("couldn't read the file for test");
	let v_s: Vec<&str> = s.split("\n").collect();
	let mut p: Parser = Parser::new(0); 
	p.read_seeds(&v_s);
	p.read_maps(&v_s);
	

	//let r1 = p1(map.clone());
	//let r2 = p2(map.clone());
	//println!("P1: {}\nP2: {}", r1, r2);
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_working() {
		let s = std::fs::read_to_string("./src/ex.txt").expect("couldn't read the file for test");
		//assert_eq!(p1(map), 35);
	}
	#[test]
	fn p1_test() {
		let s = std::fs::read_to_string("./src/input.txt").expect("couldn't read the file for test");
		//assert_eq!(p1(map), 282277027);
	}
	#[test]
	fn p2_working() {
		let s = std::fs::read_to_string("./src/ex.txt").expect("couldn't read the file for test");
		//assert_eq!(p2(map),46);
	}
}

