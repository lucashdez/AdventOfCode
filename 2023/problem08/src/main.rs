use graph::graph::*;
mod graph;


/// Returns the number of steps to get to ZZZ in the tree
/// # Arguments
/// * `s` -> String containing the full input in LF
fn p1(s: &str) -> usize {
	let (way2go, rules) = s.split_once('\n').unwrap();
	dbg!(&way2go);
	let mut traverser = GraphTraverser::new(rules);
	println!("{}", &traverser);
	let result = traverser.traverse(way2go);
	if let Some(x) = result {
		return x
	} else {
		return 0;
	}
}

fn main() {
	let str: String = std::fs::read_to_string("./src/input.txt")
		.expect("bad_luck");
	let r1 = p1(&str);
	println!("P1: {}", r1);
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

}
