use std::collections::btree_set::Intersection;
use std::ops::Range;
use std::fmt::{Display, Formatter};
use range_ext::intersect::{IntersectionExt, Intersect};

pub struct Map {
	pub key: (String, String),
	window: Vec<(Range<i64>, i64)>
}

impl Map {
	pub fn new(f: &str, t: &str) -> Self {
		Map {
			key: (f.to_string(), t.to_string()),
			window: Vec::new()
		}
	}

	pub fn insert(&mut self, r: (Range<i64>, i64)) {
		self.window.push(r);
	}

	pub fn intersect_1(&self, seed: i64) -> i64 {
		let mut result = seed;
		for w in &self.window {
			if seed >= w.0.start && seed < w.0.end {
				result = seed + w.1;
				break;
			}
		}
		result
	}

	pub fn intersect_range(&self, r: Range<i64>) -> (Option<Range<i64>>,Option<Range<i64>>,Option<Range<i64>>) {
		let mut result: (Option<Range<i64>>,Option<Range<i64>>,Option<Range<i64>>) = (None,None,None); 

		for w in &self.window {
			match r.intersect_ext(&w.0) {
				IntersectionExt::LessOverlap => {
					result = (Some(r.start..w.0.start), Some((w.0.start + w.1)..(r.end + w.1)), None);
				}, 
				IntersectionExt::Within => {
					result =  (None, Some((r.start + w.1)..(r.end + w.1)), None);
				},
				IntersectionExt::Same => {
					result =  (None, Some((r.start+w.1)..(r.end+w.1)), None);
				},
				IntersectionExt::Over => {
					result =  (Some(r.start..w.0.start), Some((w.0.start + w.1)..(w.0.end + w.1)), Some(w.0.end..r.end));
				},
				IntersectionExt::GreaterOverlap => {
					result =  (None, Some((r.start + w.1)..(w.0.end + w.1)), Some(w.0.end..r.end));
				},
				IntersectionExt::Greater | IntersectionExt::Less => { continue },
				IntersectionExt::Empty => {continue}
			}
		}
		if result == (None, None, None) {
			return (None, Some(r), None);
		}
		result
	}
	
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

	pub fn get_maps(&self) -> &Vec<Map> {
		return &self.maps
	}

	pub fn get_map(&mut self, from: String) -> Option<&mut Map> {
		let mut idx: isize = -1;
		for i in 0..self.maps.len() {
			if self.maps[i].key.0 == from {
				idx = i as isize;
				break;
			}
		}
		if idx != -1 {
			return Some(&mut self.maps[idx as usize]);
		}
		None
	}
}

impl Display for Maps {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		let maps = &self.maps;
		maps.into_iter().for_each(|m| {
			writeln!(f, "{:?}: {}", m.key, m.window.len()).unwrap();
		});
		Ok(())
	}
}
