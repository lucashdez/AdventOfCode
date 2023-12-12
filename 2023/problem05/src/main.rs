use regex::Regex;
struct Maps {
	seeds:                   Vec<u64>,
	seed_to_soil:            Vec<(u64, u64, u64)>,
	seed_to_fertilizer:      Vec<(u64, u64, u64)>,
	fertilizer_to_water:     Vec<(u64, u64, u64)>,
	water_to_light:          Vec<(u64, u64, u64)>,
	light_to_temperature:    Vec<(u64, u64, u64)>,
	temperature_to_humidity: Vec<(u64, u64, u64)>,
	humidity_to_location:    Vec<(u64, u64, u64)>
}

impl Maps {
	//{{{ fn new(input: &str) -> Self
	fn new(input: &str) -> Self {
		let rg: Regex = Regex::new(r"seeds:\s+(?<seeds>(\d+\s*)+)seed-to-soil map:\s+(?<sts>(\d+\s*)+)soil-to-fertilizer map:\s+(?<stf>(\d+\s*)+)fertilizer-to-water map:\s+(?<ftw>(\d+\s*)+)water-to-light map:\s+(?<wtl>(\d+\s*)+)light-to-temperature map:\s+(?<ltt>(\d+\s*)+)temperature-to-humidity map:\s+(?<tth>(\d+\s*)+)humidity-to-location map:\s+(?<htl>(\d+\s*)+)").expect("Couldn't create the regular expresion. Too bad!");
		let Some(captures) = rg.captures(input) else {
			panic!("Cannot get heree!!");
		};
		let tmpseeds = captures["seeds"].replace('\n', " ");
		let seeds: Vec<u64> = tmpseeds.split(' ').filter(|d| d != &"").map(|f| f.parse().unwrap()).collect();
		let sts: Vec<(u64, u64, u64)> = captures["sts"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				return (v[1].parse::<u64>().unwrap(), v[0].parse::<u64>().unwrap(), v[2].parse::<u64>().unwrap());
			}).collect();
		let stf: Vec<(u64, u64, u64)> = captures["stf"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				return (v[1].parse::<u64>().unwrap(), v[0].parse::<u64>().unwrap(), v[2].parse::<u64>().unwrap());
			}).collect();
		let ftw: Vec<(u64, u64, u64)> = captures["ftw"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				return (v[1].parse::<u64>().unwrap(), v[0].parse::<u64>().unwrap(), v[2].parse::<u64>().unwrap());
			}).collect();
		let wtl: Vec<(u64, u64, u64)> = captures["wtl"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				return (v[1].parse::<u64>().unwrap(), v[0].parse::<u64>().unwrap(), v[2].parse::<u64>().unwrap());
			}).collect();
		let ltt: Vec<(u64, u64, u64)> = captures["ltt"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				return (v[1].parse::<u64>().unwrap(), v[0].parse::<u64>().unwrap(), v[2].parse::<u64>().unwrap());
			}).collect();
		let tth: Vec<(u64, u64, u64)> = captures["tth"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				return (v[1].parse::<u64>().unwrap(), v[0].parse::<u64>().unwrap(), v[2].parse::<u64>().unwrap());
			}).collect();
		let htl: Vec<(u64, u64, u64)> = captures["htl"]
			.split('\n')
			.filter(|d| d != &"")
			.collect::<Vec<&str>>()
			.into_iter()
			.map(|f| {
				let v: Vec<&str> = f.split(' ').collect();
				return (v[1].parse::<u64>().unwrap(), v[0].parse::<u64>().unwrap(), v[2].parse::<u64>().unwrap());
			}).collect();

		Maps {
			seeds,
			seed_to_soil: sts,
			seed_to_fertilizer: stf,
			fertilizer_to_water: ftw,
			water_to_light: wtl,
			light_to_temperature: ltt,
			temperature_to_humidity: tth,
			humidity_to_location: htl
		}
	}
	//}}}

}	

fn main() {
    println!("Hello, world!");
}

mod test {
	use super::*;
	#[test]
	fn p1_working() {
		let s = std::fs::read_to_string("./src/ex.txt").expect("couldn't read the file for test");
		let map: Maps = Maps::new(s.as_str());
		dbg!(map);
		assert_eq!(1,0);
	}
}

