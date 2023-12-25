use std::ops::Range;
use regex::Regex;

pub struct Map {
	key: (String, String),
	window: Vec<Range<u64>>
}

impl Map {
	
}

pub struct Maps {
	maps: Vec<Map>
}

impl Maps {
	pub fn new() -> Self {
		Maps{ maps: Vec::new() }
	}

	pub fn insert(&mut self, m: Map) {
		self.maps.push(m);
	}

	pub fn get_map(&self, from: String) -> Option<&Map> {
		let mut idx: isize = -1;
		for i in 0..self.maps.len() {
			if self.maps[i].key.0 == from {
				idx = i as isize;
				break;
			}
		}
		if idx != -1 {
			return Some(&self.maps[idx as usize]);
		}
		None
	}
}
