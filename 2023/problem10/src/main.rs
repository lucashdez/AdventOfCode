mod lib;
use lib::{me::*, map::*};

fn p1(s: &str) -> (isize, (Conection, Movement)) {
	let mut map: Map = Map::new(s);
	let mut me: Me = Me::new(map.get_start());
	let start = map.get_start();

	let mut res = me.traverse(&map, &mut Conection::SE, &mut (-1,0));
	if res.0  {return (res.1/2, (Conection::SE, (-1,0)));}
	res = me.traverse(&map, &mut Conection::SW, &mut (-1,0));
	if res.0  {return (res.1/2, (Conection::SW, (-1,0)));}
	res = me.traverse(&map, &mut Conection::NW, &mut ( 1,0));
	if res.0  {return (res.1/2, (Conection::NW, ( 1,0)));}
	res = me.traverse(&map, &mut Conection::NE, &mut ( 1,0));
	if res.0  {return (res.1/2, (Conection::NE, ( 1,0)));}
	res = me.traverse(&map, &mut Conection::NS, &mut ( 1,0));
	if res.0  {return (res.1/2, (Conection::NS, ( 1,0)));}
	res = me.traverse(&map, &mut Conection::EW, &mut ( 0,1));
	if res.0  {return (res.1/2, (Conection::EW, ( 0,1)));}
	(res.1/2, (Conection::Ground, (0,0)))
}

trait Interior {
	fn picks(&self, boundary: usize) -> usize;
}

impl Interior for usize {
	fn picks(&self, boundary: usize) -> usize {
		self + 1 - boundary / 2
	}
}

trait Area {
	fn shoelace(&self) -> usize;
}

impl Area for Vec<(isize, isize)> {
	fn shoelace(&self) -> usize {
		(self
		 .windows(2)
		 .fold(0, |acc, matrix| 
			   acc + (matrix[0].0 * matrix[1].1) - (matrix[0].1 * matrix[1].0)
		 ) / 2).abs() as usize
	}
}

fn p2(s: &str, c: &mut Conection, m: &mut Movement) -> usize {
	let mut map: Map = Map::new(s);
	let mut me: Me = Me::new(map.get_start());
	let mut path: Vec<(isize,isize)> = Vec::new();
	path.push(me.get_pos());
	me.reg_travel(&map, &mut path, c, m);
	path.push(me.get_pos());
	path.shoelace().picks(path.len()-1)
}

fn main() {
	let input = std::fs::read_to_string("./src/input.txt")
		.expect("??");
	let mut r1 = p1(input.as_str());
	let r2 = p2(input.as_str(), &mut r1.1.0, &mut r1.1.1);
	println!("P1: {}\nP2: {}", r1.0, r2);
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_c_1() {
		let ti1 = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
		assert_eq!(p1(ti1).0, 4);
	}

	#[test]
	fn test_c_2() {
		let ti2 = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...\n";
		assert_eq!(p1(ti2).0, 8);
	}

	#[test]
	fn test_c_3() {
		let ti3 = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
	let mut r1 = p1(ti3);
		assert_eq!(p2(ti3, &mut r1.1.0, &mut r1.1.1), 4);
	}
	#[test]
	fn test_c_4() {
		let ti1 = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
		let mut r1 = p1(ti1);
		assert_eq!(p2(ti1, &mut r1.1.0, &mut r1.1.1), 1);
	}

	#[test]
	fn test_c_5() {
		let ti5 = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
		let mut r5 = p1(ti5);
		assert_eq!(p2(ti5, &mut r5.1.0, &mut r5.1.1), 10)
	}
}
