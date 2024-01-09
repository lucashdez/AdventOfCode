use graph::graph::*;
use std::collections::HashMap;
mod graph;


/// Returns the number of steps to get to ZZZ in the tree
/// # Arguments
/// * `s` -> String containing the full input in LF
fn p1(s: &str) -> usize {
	let (way2go, rules) = s.split_once('\n').unwrap();
	let mut traverser = GraphTraverser::new(rules, false);
	let result = traverser.traverse(way2go);
	if let Some(x) = result {
		return x
	} else {
		return 0;
	}
}

fn lcm(nums: &[usize]) -> usize {
	if nums.len() == 1 {
		return nums[0];
	}
	let a = nums[0];
	let b = lcm(&nums[1..]);
	a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
	if b == 0 {
		return a;
	}
	gcd_of_two_numbers(b, a % b)
}

fn p2(s: &str) -> usize {
	//ends_with para los elementos que acaban en A luego lanzar hilos sobre los elementos hasta que todos acaben en Z
	let (way2go, rules) = s.split_once('\n').unwrap();
	let c: Vec<char> = way2go.chars().filter(|c| c != &' ').collect();
	let mut travelers: Vec<&str> = Vec::new();
	let mut h: HashMap<&str, (&str, &str)> = HashMap::new();
	rules.split('\n').filter(|f| f != &"")
		.for_each(|rule| {
			let bases = rule.split_once(" = ").unwrap();
			let base = bases.0;
			let sides = bases.1.split_once(", ").unwrap();
			let left = &sides.0[1..];
			let right = &sides.1[..sides.1.len()-1];
			h.insert(base, (left, right));
		});
	for key in h.keys() {
		if key.ends_with('A') {
			travelers.push(key);
		}
	}
	let mut pos: usize = 0;
	let mut it: usize = 1;
	let mut all_z: usize = 0;
	let mut paths: Vec<usize> = Vec::new();
	let mut result = 0;
	for i in 0..travelers.len() {
		loop {
			match c[pos] {
				'L' => { travelers[i] = h.get(travelers[i]).unwrap().0 }
				'R' => { travelers[i] = h.get(travelers[i]).unwrap().1 }
				_ => unreachable!()
			}
			if travelers[i].ends_with('Z') {paths.push(it); break;}
			it += 1;
			pos += 1;
			if pos == c.len() {
				pos = 0;
			}
		}
		it = 1;
		pos = 0;
	}
	lcm(&paths)
}

fn main() {
	let str: String = std::fs::read_to_string("./src/input.txt")
		.expect("bad_luck");
	let r1 = p1(&str);
	let r2 = p2(&str);
	println!("P1: {}\nP2: {}", r1, r2);
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_1() {
		let str: String = std::fs::read_to_string("./src/ex1.txt")
			.expect("bad_luck");
		assert_eq!(p1(str.as_str()),2);
	}
	#[test]
	fn p1_2() {
		let str: String = std::fs::read_to_string("./src/ex2.txt")
			.expect("bad_luck");
		assert_eq!(p1(str.as_str()),6);
	}

	#[test]
	fn p2_test() {
		let str: String = std::fs::read_to_string("./src/ex3.txt")
			.expect("bad_luck");
		assert_eq!(p2(str.as_str()), 6);
	}
}
