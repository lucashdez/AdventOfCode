use std::ops::Range;
use regex::Regex;

#[derive(Clone)]
pub struct RangesMap {
	pub seeds: Vec<u64>,
	seeds_to_soil:           Vec<(Range<u64>, Range<u64>, u64)>,
	soil_to_fertilizer:      Vec<(Range<u64>, Range<u64>, u64)>,
	fertilizer_to_water:     Vec<(Range<u64>, Range<u64>, u64)>,
	water_to_light:          Vec<(Range<u64>, Range<u64>, u64)>,
	light_to_temperature:    Vec<(Range<u64>, Range<u64>, u64)>,
	temperature_to_humidity: Vec<(Range<u64>, Range<u64>, u64)>,
	humidity_to_location:    Vec<(Range<u64>, Range<u64>, u64)> 
}

impl RangesMap {

	// {{{ new
	pub fn new(input: &str) -> Self {
		let rg: Regex = Regex::new(r"seeds:\s+(?<seeds>(\d+\s*)+)seed-to-soil map:\s+(?<sts>(\d+\s*)+)soil-to-fertilizer map:\s+(?<stf>(\d+\s*)+)fertilizer-to-water map:\s+(?<ftw>(\d+\s*)+)water-to-light map:\s+(?<wtl>(\d+\s*)+)light-to-temperature map:\s+(?<ltt>(\d+\s*)+)temperature-to-humidity map:\s+(?<tth>(\d+\s*)+)humidity-to-location map:\s+(?<htl>(\d+\s*)+)").expect("Couldn't create the regular expresion. Too bad!");
		let Some(captures) = rg.captures(input) else {
			panic!("Cannot get heree!!");
		};
		let tmpseeds = captures["seeds"].replace('\n', " ");
		let seeds: Vec<u64> = tmpseeds.split(' ').filter(|d| d != &"").map(|f| f.parse().unwrap()).collect();
		let sts: Vec<(Range<u64>, Range<u64>, u64)> = captures["sts"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				let minit1 = v[1].parse::<u64>().unwrap();
				let minit2 = v[0].parse::<u64>().unwrap();
				let lenght = v[2].parse::<u64>().unwrap(); 
				let maxit1 = minit1 + lenght;
				let maxit2 = minit2 + lenght;
				return (minit1..maxit1, minit2..maxit2, lenght);
			}).collect();
		let stf: Vec<(Range<u64>, Range<u64>, u64)> = captures["stf"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				let minit1 = v[1].parse::<u64>().unwrap();
				let minit2 = v[0].parse::<u64>().unwrap();
				let lenght = v[2].parse::<u64>().unwrap(); 
				let maxit1 = minit1 + lenght;
				let maxit2 = minit2 + lenght;
				return (minit1..maxit1, minit2..maxit2, lenght);
			}).collect();
		let ftw: Vec<(Range<u64>, Range<u64>, u64)> = captures["ftw"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				let minit1 = v[1].parse::<u64>().unwrap();
				let minit2 = v[0].parse::<u64>().unwrap();
				let lenght = v[2].parse::<u64>().unwrap(); 
				let maxit1 = minit1 + lenght;
				let maxit2 = minit2 + lenght;
				return (minit1..maxit1, minit2..maxit2, lenght);
			}).collect();
		let wtl: Vec<(Range<u64>, Range<u64>, u64)> = captures["wtl"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				let minit1 = v[1].parse::<u64>().unwrap();
				let minit2 = v[0].parse::<u64>().unwrap();
				let lenght = v[2].parse::<u64>().unwrap(); 
				let maxit1 = minit1 + lenght;
				let maxit2 = minit2 + lenght;
				return (minit1..maxit1, minit2..maxit2, lenght);
			}).collect();
		let ltt: Vec<(Range<u64>, Range<u64>, u64)> = captures["ltt"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				let minit1 = v[1].parse::<u64>().unwrap();
				let minit2 = v[0].parse::<u64>().unwrap();
				let lenght = v[2].parse::<u64>().unwrap(); 
				let maxit1 = minit1 + lenght;
				let maxit2 = minit2 + lenght;
				return (minit1..maxit1, minit2..maxit2, lenght);
			}).collect();
		let tth: Vec<(Range<u64>, Range<u64>, u64)> = captures["tth"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				let minit1 = v[1].parse::<u64>().unwrap();
				let minit2 = v[0].parse::<u64>().unwrap();
				let lenght = v[2].parse::<u64>().unwrap(); 
				let maxit1 = minit1 + lenght;
				let maxit2 = minit2 + lenght;
				return (minit1..maxit1, minit2..maxit2, lenght);
			}).collect();
		let htl: Vec<(Range<u64>, Range<u64>, u64)> = captures["htl"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				let minit1 = v[1].parse::<u64>().unwrap();
				let minit2 = v[0].parse::<u64>().unwrap();
				let lenght = v[2].parse::<u64>().unwrap(); 
				let maxit1 = minit1 + lenght;
				let maxit2 = minit2 + lenght;
				return (minit1..maxit1, minit2..maxit2, lenght);
			}).collect();

		RangesMap {
			seeds,
			seeds_to_soil: sts,
			soil_to_fertilizer: stf,
			fertilizer_to_water: ftw,
			water_to_light: wtl,
			light_to_temperature: ltt,
			temperature_to_humidity: tth,
			humidity_to_location: htl
		}
	}
	// }}}

	fn intersect(r1: &Range<u64>, r2: &Range<u64>) -> Option<Range<u64>> {
		let start = r1.start.max(r2.start);
		let end = r1.end.min(r2.end);
		if start <= end {
			Some(start..end)
		} else {
			None
		}
	}

	//{{{ seed_to_location
	pub fn seed_to_location (&self, seed: u64) -> u64 {
		let mut seed_range = seed..seed;
		for r2 in &self.seeds_to_soil {
			if let Some(x) =  RangesMap::intersect(&seed_range ,&r2.0) {
				let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
				seed_range = (x.start as i64-factor)as u64..(x.start as i64-factor) as u64;
				break;
			}
		}

		for r2 in &self.soil_to_fertilizer {
			if let Some(x) =  RangesMap::intersect(&seed_range ,&r2.0) {
				let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
				seed_range = (x.start as i64-factor)as u64..(x.start as i64-factor) as u64;
				break;
			}
		}

		for r2 in &self.fertilizer_to_water {
			if let Some(x) =  RangesMap::intersect(&seed_range ,&r2.0) {
				let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
				seed_range = (x.start as i64-factor)as u64..(x.start as i64-factor) as u64;
				break;
			}
		}

		for r2 in &self.water_to_light {
			if let Some(x) =  RangesMap::intersect(&seed_range ,&r2.0) {
				let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
				seed_range = (x.start as i64-factor)as u64..(x.start as i64-factor) as u64;
				break;
			}
		}

		for r2 in &self.light_to_temperature {
			if let Some(x) =  RangesMap::intersect(&seed_range ,&r2.0) {
				let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
				seed_range = (x.start as i64-factor)as u64..(x.start as i64-factor) as u64;
				break;
			}
		}

		for r2 in &self.temperature_to_humidity {
			if let Some(x) =  RangesMap::intersect(&seed_range ,&r2.0) {
				let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
				seed_range = (x.start as i64-factor)as u64..(x.start as i64-factor) as u64;
				break;
			}
		}

		for r2 in &self.humidity_to_location {
			if let Some(x) =  RangesMap::intersect(&seed_range ,&r2.0) {
				let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
				seed_range = (x.start as i64-factor)as u64..(x.start as i64-factor) as u64;
				break;
			}
		}
		seed_range.start
	}
	//}}}

	// {{{ ranges_to_location
	pub fn ranges_to_location(&self, in_range: Range<u64>) -> u64 {
		let mut ranges: Vec<Range<u64>> = vec![in_range];
		let mut more_ranges: Vec<Range<u64>> = Vec::new();
		let mut inter: bool = false;
		for range in &ranges {
			for r2 in &self.seeds_to_soil {
				if let Some(x) =  RangesMap::intersect(&range, &r2.0) {
					inter = true;
					let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
					let new_start: u64 = (x.start as i64 - (factor)) as u64;
					let lenght = x.end-x.start;
					more_ranges.push(new_start..new_start+lenght);
					if range.start < x.start {
						more_ranges.push(range.start..x.start);
					}
					if range.end > x.end {
						more_ranges.push(x.end+1..range.end);
					}
					break;
				}
			}
			if !inter {
				more_ranges.push(range.clone());
			}
			inter = false;
		}
		ranges = more_ranges.clone();
		more_ranges.clear();

		for range in &ranges {
			for r2 in &self.soil_to_fertilizer {
				if let Some(x) =  RangesMap::intersect(&range, &r2.0) {
					inter = true;
					let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
					let new_start: u64 = (x.start as i64 - (factor)) as u64;
					let lenght = x.end-x.start;
					more_ranges.push(new_start..new_start+lenght);
					if range.start < x.start {
						more_ranges.push(range.start..x.start);
					}
					if range.end > x.end {
						more_ranges.push(x.end+1..range.end);
					}
					break;
				}
			}
			if !inter {
				more_ranges.push(range.clone());
			}
			inter = false
		}
		ranges = more_ranges.clone();
		more_ranges.clear();

		for range in &ranges {
			for r2 in &self.fertilizer_to_water {
				if let Some(x) =  RangesMap::intersect(&range, &r2.0) {
					inter = true;
					let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
					let new_start: u64 = (x.start as i64 - (factor)) as u64;
					let lenght = x.end-x.start;
					more_ranges.push(new_start..new_start+lenght);
					if range.start < x.start {
						more_ranges.push(range.start..x.start);
					}
					if range.end > x.end {
						more_ranges.push(x.end+1..range.end);
					}
					break;
				}
			}
			if !inter {
				more_ranges.push(range.clone());
			}
			inter = false
		}
		ranges = more_ranges.clone();
		more_ranges.clear();

		for range in &ranges {
			for r2 in &self.water_to_light {
				if let Some(x) =  RangesMap::intersect(&range, &r2.0) {
					inter = true;
					let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
					let new_start: u64 = (x.start as i64 - (factor)) as u64;
					let lenght = x.end-x.start;
					more_ranges.push(new_start..new_start+lenght);
					if range.start < x.start {
						more_ranges.push(range.start..x.start);
					}
					if range.end > x.end {
						more_ranges.push(x.end+1..range.end);
					}
					break;
				}
			}
			if !inter {
				more_ranges.push(range.clone());
			}
			inter = false
		}
		ranges = more_ranges.clone();
		more_ranges.clear();

		for range in &ranges {
			for r2 in &self.light_to_temperature {
				if let Some(x) =  RangesMap::intersect(&range, &r2.0) {
					inter = true;
					let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
					let new_start: u64 = (x.start as i64 - (factor)) as u64;
					let lenght = x.end-x.start;
					more_ranges.push(new_start..new_start+lenght);
					if range.start < x.start {
						more_ranges.push(range.start..x.start);
					}
					if range.end > x.end {
						more_ranges.push(x.end+1..range.end);
					}
					break;
				}
			}
			if !inter {
				more_ranges.push(range.clone());
			}
			inter = false
		}
		ranges = more_ranges.clone();
		more_ranges.clear();

		for range in &ranges {
			for r2 in &self.temperature_to_humidity {
				if let Some(x) =  RangesMap::intersect(&range, &r2.0) {
					inter = true;
					let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
					let new_start: u64 = (x.start as i64 - (factor)) as u64;
					let lenght = x.end-x.start;
					more_ranges.push(new_start..new_start+lenght);
					if range.start < x.start {
						more_ranges.push(range.start..x.start);
					}
					if range.end > x.end {
						more_ranges.push(x.end+1..range.end);
					}
					break;
				}
			}
			if !inter {
				more_ranges.push(range.clone());
			}
			inter = false
		}
		ranges = more_ranges.clone();
		more_ranges.clear();

		for range in &ranges {
			for r2 in &self.humidity_to_location {
				if let Some(x) =  RangesMap::intersect(&range, &r2.0) {
					inter = true;
					let factor: i64 = r2.0.start as i64 - r2.1.start as i64;
					let new_start: u64 = (x.start as i64 - (factor)) as u64;
					let lenght = x.end-x.start;
					more_ranges.push(new_start..new_start+lenght);
					if range.start < x.start {
						more_ranges.push(range.start..x.start);
					}
					if range.end > x.end {
						more_ranges.push(x.end+1..range.end);
					}
					break;
				}
			}
			if !inter {
				more_ranges.push(range.clone());
			}
			inter = false
		}
		ranges = more_ranges.clone();
		more_ranges.clear();
		ranges.into_iter().min_by(|a,b| a.start.cmp(&b.start)).unwrap().start
	}
	// }}}
	
}
