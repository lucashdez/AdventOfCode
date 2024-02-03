mod lib;
use lib::map::*;

struct Augments {
	rows: usize,
	cols: usize,
}

type Coord = (usize, usize);

fn distance_in(expansion_r: &Vec<usize>
			   , expansion_c: &Vec<usize>
			   , from: Coord
			   , to: Coord
			   , times: usize) -> usize {
	let (row_min, row_max) = (from.0.min(to.0), from.0.max(to.0));
	let (col_min, col_max) = (from.1.min(to.1), from.1.max(to.1));

	let d_row = row_max - row_min;
	let d_col = col_max - col_min;

	let row_prod = expansion_r.iter().filter(|&row| *row >= row_min && *row <= row_max).count();
	let col_prod = expansion_c.iter().filter(|&col| *col >= col_min && *col <= col_max).count();

	d_col + col_prod * (times - 1) + d_row + row_prod * (times - 1)
}

fn p1(map: &mut Map, times: usize) -> usize {
	let mut v_rows: Vec<usize> = Vec::new();
	let mut v_cols: Vec<usize> = Vec::new();
	for i in 0..map.get_v().len() {
		if map.get_v()[i].iter().all(|x| *x == 0) {
			v_rows.push(i);
		}
	}

	for i in 0..map.get_v()[0].len() {
		let mut t: bool = true;
		for j in 0..map.get_v().len() {
			if map.get_v()[j][i] == 1 {
				t = false;
				break;
			}
		} 
		if t {
			v_cols.push(i);
		}
	}

	let mut galaxies: Vec<(usize, usize)> = Vec::new();
	for i in 0..map.get_v().len() {
		for j in 0..map.get_v()[i].len() {
			if map.get_v()[i][j] == 1 {
				galaxies.push((i,j));
			}
		}
	}

	let mut sum = 0;
	for i in 0..galaxies.len() - 1 {
		for j in i+1..galaxies.len() {
			sum += distance_in(&v_rows, &v_cols, galaxies[i], galaxies[j], times);
		}
	}
	sum
}

fn main() {
	let input = std::fs::read_to_string("./src/input.txt")
		.expect("Couldn't read");
	let mut map = Map::new(&input);
	let r1 = p1(&mut map, 2);
	let r2 = p1(&mut map, 1000000);
	println!("P1:{}\nP2:{}", r1, r2);
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_c_1() {
		let t1 = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
		let mut m1 = Map::new(t1);
		assert_eq!(p1(&mut m1, 2),374);
	}
	#[test]
	fn text_c_2() {
		let t1 = std::fs::read_to_string("./src/input.txt").expect("Couldn't read");
		let mut m1 = Map::new(&t1);

		assert_eq!(p1(&mut m1, 2), 9805264);
	}


	#[test]
	fn test_c_3() {
		let t1 = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
		let mut m1 = Map::new(t1);
		assert_eq!(p1(&mut m1, 10),1030);
	}

	#[test]
	fn test_c_4() {
		let t1 = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
		let mut m1 = Map::new(t1);
		assert_eq!(p1(&mut m1, 100), 8410);
	}
}
