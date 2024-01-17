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

fn p2(s: &str, c: &mut Conection, m: &mut Movement) -> isize{
	let mut map: Map = Map::new(s);
	let mut me: Me = Me::new(map.get_start());
	let mut path: Vec<(usize,usize)> = Vec::new();
	me.reg_travel(&map, &mut path, c, m);

	let my = map.max_i();
	let mx = map.max_j();

	let mut count = 0;
	let mut str: String = String::from("\n");
	let mut contained = false;
	let mut cantcontainmore = false;
	for i in 0..my as usize {
		contained = false;
		cantcontainmore = false;
		for j in 0..mx as usize {
			if path.contains(&(i,j)) {
				if !contained {
					cantcontainmore = true;
				}
				contained = true;
				str.push('-')
			} else if contained && !cantcontainmore {
				str.push('i')
			}
			else {
				contained = false;
				str.push(' ')
			}
		}
		str.push('\n')
	}
	println!("{}", str);
	count
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
		let ti3 = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L\n";
		let mut r1 = p1(ti3);
		assert_eq!(p2(ti3, &mut r1.1.0, &mut r1.1.1), 10)
	}
}
