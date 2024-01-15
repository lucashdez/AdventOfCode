mod lib;
use lib::{me::*, map::*};

fn p1(s: &str) -> usize {
	let map: Map = Map::new(s);
	let mut me: Me = Me::new(map.get_start());
	dbg!(map);
	0
}

fn main() {
	let input = std::fs::read_to_string("./src/input.txt")
		.expect("??");
	let r1 = p1(input.as_str());
	println!("P1: {r1}");
    
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_c_1() {
		let ti1 = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
		assert_eq!(p1(ti1), 4);
	}

	#[test]
	fn test_c_2() {
		let ti2 = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...\n";
		assert_eq!(p1(ti2), 8);
	}
}
