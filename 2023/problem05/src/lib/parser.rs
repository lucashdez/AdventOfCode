use crate::Maps;

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

	pub fn read_seeds(&mut self, s: &Vec<&str>) -> Vec<u64> {
		let line: usize = self.line;
		let it = s[line].split_once("seeds:").unwrap().1;
		self.line += 1;
		let result: Vec<u64> = it.split(' ').filter(|f| { f != &"" }).map(|n| {n.parse().unwrap()}).collect();
		dbg!(&result);
		return result;
	}

	pub fn read_maps(&mut self, s: &Vec<&str>) -> Maps {
		while self.line < s.len() {

			self.line += 1;
		}
		todo!()
	}
}
