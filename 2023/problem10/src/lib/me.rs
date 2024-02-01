use crate::*;

pub type Coord = (isize, isize);
pub type Movement = (isize, isize);
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Conection {
	NS, EW, NE, NW, SW, SE, Ground, Start, Wall
}


impl Conection {
	fn next_position(p: &Movement, a: &Conection) -> Movement {
		match (p,a) {
			((-1, _), Conection::NS) => {(-1, 0)} // _ -> N -> S
			(( 1, _), Conection::NS) => {( 1, 0)} // _ -> S -> N
			(( _,-1), Conection::EW) => {( 0,-1)} // _ -> E -> W
			(( _, 1), Conection::EW) => {( 0, 1)} // _ -> W -> E
			(( 1, _), Conection::NE) => {( 0, 1)} // _ -> N -> E
			(( _,-1), Conection::NE) => {(-1, 0)} // _ -> E -> N
			(( 1, _), Conection::NW) => {( 0,-1)} // _ -> N -> W 
			(( _, 1), Conection::NW) => {(-1, 0)} // _ -> W -> N
			((-1, _), Conection::SW) => {( 0,-1)} // _ -> S -> W
			(( _, 1), Conection::SW) => {( 1, 0)} // _ -> W -> S
			((-1, _), Conection::SE) => {( 0, 1)} // _ -> S -> E
			(( _,-1), Conection::SE) => {( 1, 0)} // _ -> E -> S
			_ => {(0,0)}
		}
	}
}


pub struct Me {
	pos: Coord
}

impl Me {
	pub fn new(c: Coord) -> Self {
		Me {
			pos: c
		}
	}

	pub fn get_pos(&self) -> Coord {
		self.pos
	}

	fn mov(&mut self,map: &Map, start: &Coord, prev: &mut Movement, act: &mut Conection) -> (bool, isize) {
		let mut count = 0;
		loop {
			let mov = Conection::next_position(&prev, act);
			self.pos = (mov.0 + self.pos.0, self.pos.1 + mov.1);
			if self.pos == *start {
				count += 1;
				break;
			} else if self.pos.0 < 0 || self.pos.0 > map.max_i() || self.pos.1 < 0 || self.pos.1 > map.max_j() || mov == (0,0) {
				return (false, -1);
			} else {
				*prev = mov;
				*act = map.get(self.pos).clone();
				count += 1;
			}	
		}
		(true, count)
	} 

	pub fn traverse(&mut self, map: &Map, actual_conection: &mut Conection, prev: &mut Movement) -> (bool, isize) {
		let start = self.pos;
		let count = self.mov(map, &start, prev, actual_conection);
		count
	}
	
	pub fn reg_travel(&mut self, map: &Map, reg: &mut Vec<(isize, isize)>, act: &mut Conection, prev: &mut Movement) {
		let start = self.pos;
		reg.push((start.0, start.1));
		let mut count = 0;
		loop {
			let mov = Conection::next_position(&prev, act);
			self.pos = (mov.0 + self.pos.0, self.pos.1 + mov.1);
			if self.pos == start {
				break;
			} else {
				reg.push((self.pos.0, self.pos.1));
				*prev = mov;
				*act = map.get(self.pos).clone();
			}	
		}
	}
}
