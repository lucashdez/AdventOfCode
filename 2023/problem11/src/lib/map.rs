pub struct Map {
	map: Vec<Vec<isize>>
}

impl Map {
	pub fn new(s: &str) -> Self {
		let mut v: Vec<Vec<isize>> = s
			.split('\n')
			.filter(|d| *d != "")
			.map(|line| line
				 .chars()
				 .map(|c| {
					 if c == '#' {
						 return 1;
					 }
					 0
				 })
				 .collect())
			.collect();
		Map {
			map: v
		}
	}

	pub fn get_v(&self) -> Vec<Vec<isize>> {
		self.map.clone()
	}

	pub fn max_rows(&self) -> usize {
		self.map.len()
	}

	pub fn max_cols(&self) -> usize {
		self.map[0].len()
	}

	pub fn add_row(&mut self, row_pos: usize) {
		let mut new_row = self.map[0].clone();
		new_row.iter_mut().for_each(|i| *i = 0);
		self.map.insert(row_pos, new_row);
	}

	pub fn add_col(&mut self, col_pos: usize) {
		for i in 0..self.map.len() {
			self.map[i].insert(col_pos, 0);
		}
	}
}


impl std::fmt::Debug for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		let mut s: String = String::from("\n");
		self.map.iter().for_each(|line| {
			line.iter().for_each(|i| {
				if *i == 1 {
					s.push('#');
				} else {
					s.push('.');
				}
			});
			s.push('\n');
		});
		write!(f, "{}", s).expect("Something");
		Ok(())
	}
}
