use crate::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Conection {
	NS, EW, NE, NW, SW, SE, Ground, Start, Wall
}

impl Conection {
}

pub type Coord = (usize, usize);

pub struct Me {
	pos: Coord
}

impl Me {
	pub fn new(c: Coord) -> Self {
		Me {
			pos: c
		}
	}

	fn mov(&mut self, prev: Conection) -> isize {
		0
	} 

	pub fn traverse(&mut self, map: &Map, actual_conection: Conection , goto: u8) -> usize {
		let new_con = map.get(self.pos);

		if goto == 0 {
			return 0;
		} else {
			return 0;
		}
	}
}
