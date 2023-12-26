use crate::{Maps, Map};

pub struct Parser {
	parser: isize,
	line: usize
}

impl Parser {
	pub fn new(h: isize) -> Self {
		Parser {
			parser: h,
			line: 0
		}
	}

	pub fn read_seeds(&mut self, s: &Vec<&str>) -> Vec<i64> {
		let line: usize = self.line;
		let it = s[line].split_once("seeds:").unwrap().1;
		self.line += 1;
		let result: Vec<i64> = it.split(' ').filter(|f| { f != &"" }).map(|n| {n.parse().unwrap()}).collect();
		return result;
	}

	pub fn read_maps(&mut self, s: &Vec<&str>) -> Maps {
		let mut from: String = String::from("");
		let mut maps: Maps = Maps::new();
		while self.line < s.len() {
			if s[self.line].contains("-to-") {
				let trs: &str = s[self.line].split(" ").collect::<Vec<&str>>()[0];
				let r = trs.trim().split_once("-to-")
					.expect("There are no names?");
				from = r.0.to_string();
				let tmpmap: Map = Map::new(&from, &r.1.to_string());
				maps.insert(tmpmap);
			} else {
				if let Some(map) = &mut maps.get_map(from.clone()) {
					let trs = s[self.line].split(' ').collect::<Vec<&str>>();
					if trs.len() > 2 {
						let f: i64 = trs[1].trim().parse().unwrap();
						let t: i64 = trs[0].trim().parse().unwrap();
						let s: i64 = trs[2].trim().parse().unwrap();
						let range = f..f+s;
						let factor = t - f;
						map.insert((range, factor));
					}
				} else {
				}
			}
			self.line += 1;
		}
		println!("{}",maps);
		return maps;
	}
}
