use std::ops::Range;


#[derive(Debug, PartialEq)]
enum MapCell{
	WALL,
	CELL(bool),
}

#[derive(Debug)]
enum Direction {
	N,W,S,E
}


fn print_map(map: &Vec<Vec<MapCell>>, traveler: &(usize, usize, Direction)) {
	map.iter().enumerate().for_each(|(i, x)| {
		let mut s: String = String::from("");
		x.iter().enumerate().for_each(|(j, c)| {
			let real_char: char = if traveler.0 == i && traveler.1 == j {
				match traveler.2 {
					Direction::N => '^'
						, Direction::W => '<'
						, Direction::S => 'v'
						, Direction::E => '>'
				}
				
			} else {
				match c {
					MapCell::WALL => {'#'}
					, MapCell::CELL(false) => {'.'}
					, MapCell::CELL(true) => {'X'}
					
				}};
			s.push(real_char);
		});
		println!("{s}");
	});
}

fn redirect(t: &mut  (usize, usize, Direction) ) {
	match t.2 {
		Direction::N=>{t.2 = Direction::E}
		, Direction::W=>{t.2 = Direction::N}
		, Direction::S=>{t.2 = Direction::W}
		, Direction::E=>{t.2 = Direction::S}
	}
}

fn walk(map: &mut Vec<Vec<MapCell>>, t: &mut  (usize, usize, Direction)) -> bool {
	match t.2 {
		Direction::N=>{
			let mut f: usize = 0;
			for i in 0..t.0 {
				if map[i][t.1] == MapCell::WALL {
					f = i;
				}
			}
			for i in f+1..t.0 {
				map[i][t.1] = MapCell::CELL(true);
			}
			if f == 0 && (map[f][t.1] == MapCell::CELL(false)
						  ||map[f][t.1] == MapCell::CELL(true)) {	// The traveler will die in the dephts of the array
				return false;
			}
			redirect(t);
			t.0 = f+1;
		}
		, Direction::W=>{
			let mut f: usize = 0;
			for j in 0..t.1 {
				if map[t.0][j] == MapCell::WALL {
					f = j;
				}
			}
			for j in f+1..t.1 {
				map[t.0][j] = MapCell::CELL(true);
			}
			if f == 0 && (map[t.0][f] == MapCell::CELL(false)
						  || map[t.0][f] == MapCell::CELL(true)) {	// The traveler will die in the dephts of the array
				return false;
			}
			redirect(t);
			t.1 = f+1;
		}
		, Direction::S=>{
			let mut f: usize = 0;
			let max_i = map.len();
			for i in t.0..max_i {
				f = i;
				if map[i][t.1] == MapCell::WALL {
					break;
				}
				map[i][t.1] = MapCell::CELL(true);
			}
			if f == max_i-1 && (map[f][t.1] == MapCell::CELL(false)
								|| map[f][t.1] == MapCell::CELL(true)) {	// The traveler will die in the dephts of the array
				return false;
			}
			redirect(t);
			t.0 = if f> 0 {f-1} else {f};
		}
		, Direction::E=>{
			let mut f: usize = 0;
			let max_j = map[0].len();
			for j in t.1..max_j {
				f = j;
				if map[t.0][j] == MapCell::WALL {
					break;
				}
				map[t.0][j] = MapCell::CELL(true);
			}
			if f == max_j-1 && (map[t.0][f] == MapCell::CELL(false)
								|| map[t.0][f] == MapCell::CELL(true)) {	// The traveler will die in the dephts of the array
				return false;
			}
			redirect(t);
			t.1 = if f > 0 {f-1} else {f}; 
		}

	}
	true
}


fn main() {
	let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
	let input = std::fs::read_to_string("input/input.txt").expect("couldn't read");
	#[cfg(windows)]
	let input = input.replace("\r","");

	let mut traveler: (usize, usize, Direction) = (0,0, Direction::N);

	let mut map: Vec<Vec<MapCell>> = input.lines()
		.enumerate().map(|(i, l)| l.chars().enumerate()
						 .map(|(j, c)| match c {
							 '#' => {MapCell::WALL}
							 , '.' => {MapCell::CELL(false)}
							 , '^' => { traveler = (i, j, Direction::N); MapCell::CELL(false)}
							 , _ => {unreachable!()}
						 }).collect::<Vec<MapCell>>()).collect::<Vec<_>>();

	map[traveler.0][traveler.1] = MapCell::CELL(true);
	while walk(&mut map, &mut traveler) {} 
	let p1: usize = map.iter().map(|l| l.iter()
							.map(|c| match c {
								MapCell::CELL(true) => 1,
								_ => {0}
							}).sum::<usize>()).sum();
	
    println!("p1: {p1}");
}
