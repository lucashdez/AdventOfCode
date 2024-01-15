use crate::*;

pub struct Map {
	map: Vec<Vec<Conection>>,
}

impl Map {

	fn get_dir(c: char) -> Conection {
		match c {
			'|' => {Conection::NS},
			'-' => {Conection::EW},
			'L' => {Conection::NE},
			'J' => {Conection::NW},
			'7' => {Conection::SW},
			'F' => {Conection::SE},
			'.' => {Conection::Ground},
			'S' => {Conection::Start},
			_ => {Conection::Wall}
		}
	}
	pub fn new(s: &str) -> Self {
		let new_map = s
			.split('\n')
			.map(|line| {
				let tline: Vec<Conection> = line.chars().map(|c| {Map::get_dir(c)}).collect();
				return tline;
			}).collect();

		Map {
			map: new_map
		}
	}

	pub fn get_start(&self) -> Coord {
		for i in 0..self.map.len() {
			for j in 0..self.map[i].len() {
				if self.map[i][j] == Conection::Start {
					return (i,j);
				}
			}
		}
		unreachable!()
	}
}


impl std::fmt::Display for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		let mut s: &str = "";
		write!(f, "{}", s).expect("fmt??");

		Ok(())
	}
}

impl std::fmt::Debug for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		Ok(())
	}
}
