
pub struct Parser {
	handle: usize
}

impl Parser {
	pub fn new(i: usize) -> Self {
		Parser {handle: i}
	}

	pub fn parse2(&self, s: &str) -> (i64, i64) {
		let v = s.split('\n').filter(|f| f != &"").collect::<Vec<&str>>();
		let mut result: (i64, i64) = (0, 0); 
		if v.len() == 2 {
			let (_, a) =  v[0].split_once("Time:").unwrap(); 
			let times: i64 = a.split(' ')
				.filter(|f| f != &"")
				.map(|v| v.trim())
				.collect::<Vec<&str>>()
				.join("")
				.parse()
				.unwrap();
			let (_, b) = v[1].split_once("Distance:").unwrap();
			let distances: i64 = b.split(' ')
				.filter(|f| f != &"")
				.map(|v| v.trim())
				.collect::<Vec<&str>>()
				.join("")
				.parse()
				.unwrap();

			result = (times, distances);
		} else {
			panic!("SOMETHING WENT WRONG");
		}
		result
	}

	pub fn parse(&self,  s: &str) -> Vec<(i64, i64)> {
		let v = s.split('\n').filter(|f| f != &"").collect::<Vec<&str>>();
		let mut result: Vec<(i64, i64)> = Vec::new();
		if v.len() == 2 {
			let (_, a) =  v[0].split_once("Time:").unwrap(); 
			let times: Vec<i64> = a.split(' ')
				.filter(|f| f != &"")
				.map(|v| v.trim().parse().unwrap())
				.collect();
			let (_, b) = v[1].split_once("Distance:").unwrap();
			let distances: Vec<i64> = b.split(' ')
				.filter(|f| f != &"")
				.map(|v| v.trim().parse().unwrap())
				.collect();
			for i in 0..times.len() {
				result.push((times[i], distances[i]));
			}
		} else {
			panic!("SOMETHING WENT WRONG");
		}
		result
	}
}


