use graph::graph::*;
mod graph;


/// Returns the number of steps to get to ZZZ in the tree
/// # Arguments
/// * `s` -> String containing the full input in LF
fn p1(s: &str) -> usize {
	let (way2go, rules) = s.split_once('\n').unwrap();
	let mut traverser = GraphTraverser::new(rules);
	let result = traverser.traverse(way2go);
	if let Some(x) = result {
		return x
	} else {
		return 0;
	}
}

fn p2(s: &str) -> usize {
	//ends_with para los elementos que acaban en A luego lanzar hilos sobre los elementos hasta que todos acaben en Z
	let (way2go, rules) = s.split_once('\n').unwrap();
	let mut graph = GraphTraverser::new(rules);
	let mut travelers = graph.get_travelers();
	println!("{:?}",travelers);
	0
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
