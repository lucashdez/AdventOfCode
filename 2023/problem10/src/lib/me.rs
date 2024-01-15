use crate::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Conection {
	NS, EW, NE, NW, SW, SE, Ground, Start, Wall
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
	pub fn mov(&mut self, map: Map) {
		let new_con = map.get(self.pos);
	}
}
