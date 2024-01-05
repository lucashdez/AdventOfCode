mod graph;


/// Returns the number of steps to get to ZZZ in the tree
/// # Arguments
/// * `s` -> String containing the full input in LF
fn p1(s: &str) -> usize {
	0
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_1() {
		let str: String = std::fs::read_to_string("./src/ex1.txt")
			.expect("bad_luck");
		assert_eq!(p1(str.as_str()),1);
	}
	#[test]
	fn p1_2() {
		let str: String = std::fs::read_to_string("./src/ex2.txt")
			.expect("bad_luck");
		assert_eq!(p1(str.as_str()),1);
	}

}
