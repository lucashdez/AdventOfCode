mod lib;
use lib::{
	maps::*,
	parser::*
};
use std::ops::Range;
	

fn p1(s: &Vec<i64>, m: &Maps) -> i64 {
	let mut results: Vec<i64> = s.clone();
	let maps = m.get_maps();
	let size = maps.len();
	for vi in 0..results.len() {
		for i in 0..size {
			results[vi] = maps[i].intersect_1(results[vi]);
		}
	}
	results.into_iter().min().unwrap()
}


fn p2(s: &Vec<i64>, m: &Maps) -> i64 {
	let mut all_ranges: Vec<Range<i64>> = Vec::new();
	let mut tmp_ranges: Vec<Range<i64>> = Vec::new();
	let maps = m.get_maps();
	let mut i = 0;
	while i < s.len() {
		let maxit = s[i] + s[i+1];
		let minit = s[i];
		all_ranges.push(minit..maxit);
		i += 2;
	}

	for i in 0..maps.len() {
		for range in &all_ranges {
			let abc = maps[i].intersect_range(range.clone());
			if let Some(a) = abc.0 { tmp_ranges.push(a); }
			if let Some(b) = abc.1 { tmp_ranges.push(b); }
			if let Some(c) = abc.2 { tmp_ranges.push(c); }
		}
		all_ranges.clear();
		all_ranges.append(&mut tmp_ranges);
	}
	all_ranges.into_iter().min_by(|a,b| {a.start.cmp(&b.start)}).unwrap().start
}


fn main() {
	let s = std::fs::read_to_string("./src/input.txt").expect("couldn't read the file for test");
	let v_s: Vec<&str> = s.split("\n").collect();
	let mut p: Parser = Parser::new(0); 
	let seeds: Vec<i64> = p.read_seeds(&v_s);
	let maps: Maps = p.read_maps(&v_s);
	

	let r1 = p1(&seeds, &maps);
	let r2 = p2(&seeds, &maps); 
	println!("P1: {}\nP2: {}", r1, r2);
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_working() {
		let s = std::fs::read_to_string("./src/ex.txt").expect("couldn't read the file for test");
		let v_s: Vec<&str> = s.split("\n").collect();
		let mut p: Parser = Parser::new(0); 
		let seeds: Vec<i64> = p.read_seeds(&v_s);
		let maps: Maps = p.read_maps(&v_s);
		assert_eq!(p1(&seeds, &maps), 35);
	}
	#[test]
	fn p1_test() {
		let s = std::fs::read_to_string("./src/input.txt").expect("couldn't read the file for test");
		let v_s: Vec<&str> = s.split("\n").collect();
		let mut p: Parser = Parser::new(0); 
		let seeds: Vec<i64> = p.read_seeds(&v_s);
		let maps: Maps = p.read_maps(&v_s);
		assert_eq!(p1(&seeds, &maps), 282277027);
	}
	#[test]
	fn p2_working() {
		let s = std::fs::read_to_string("./src/ex.txt").expect("couldn't read the file for test");
		let v_s: Vec<&str> = s.split("\n").collect();
		let mut p: Parser = Parser::new(0); 
		let seeds: Vec<i64> = p.read_seeds(&v_s);
		let maps: Maps = p.read_maps(&v_s);
		assert_eq!(p2(&seeds, &maps),46);
	}
}

