use std::f64::INFINITY;

use regex::Regex;
enum TransformType {
	ToSoil,
	ToFertilizer,
	ToWater,
	ToLight,
	ToTemperature,
	ToHumidity,
	ToLocation
}

struct Maps {
	seeds:                   Vec<u64>,
	seed_to_soil:            Vec<(u64, u64, u64)>,
	soil_to_fertilizer:      Vec<(u64, u64, u64)>,
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
			soil_to_fertilizer: stf,
			fertilizer_to_water: ftw,
			water_to_light: wtl,
			light_to_temperature: ltt,
			temperature_to_humidity: tth,
			humidity_to_location: htl
		}
	}
	//}}}

	fn internal_process(v: &mut Vec<u64>, transform: &Vec<(u64, u64, u64)>) {
		for i in 0..v.len() {
			for t in transform {
				let factor: i64 = t.0 as i64 - t.1 as i64;
				if v[i] >= t.0 && v[i] < t.0+t.2 {
					v[i] = (v[i] as i64 - (factor)) as u64;
					break;
				}
			}
		}
	}

	fn process_one(&self, v: &mut Vec<u64>, t: TransformType) {
		match t {
			TransformType::ToSoil        => Maps::internal_process(v, &self.seed_to_soil),
			TransformType::ToFertilizer  => Maps::internal_process(v, &self.soil_to_fertilizer),
			TransformType::ToWater       => Maps::internal_process(v, &self.fertilizer_to_water),
			TransformType::ToLight       => Maps::internal_process(v, &self.water_to_light),
			TransformType::ToTemperature => Maps::internal_process(v, &self.light_to_temperature),
			TransformType::ToHumidity    => Maps::internal_process(v, &self.temperature_to_humidity),
			TransformType::ToLocation    => Maps::internal_process(v, &self.humidity_to_location),
		}
	}

	fn from_seed(seed: u64, transform: &Vec<(u64, u64, u64)>) -> u64 {
		for t in transform {
				let factor: i64 = t.0 as i64 - t.1 as i64;
				if seed >= t.0 && seed < t.0+t.2 {
					return (seed as i64 - (factor)) as u64;
				}
		}
		seed
	}

	fn process_range(from: u64, to: u64) -> u64 {
		let mut min: u64 = std::u64::MAX;
		return min;
	}
}	


fn p1(map: &Maps) -> u64 {
	let mut seeds = map.seeds.clone();
	map.process_one(&mut seeds, TransformType::ToSoil);
	map.process_one(&mut seeds, TransformType::ToFertilizer);
	map.process_one(&mut seeds, TransformType::ToWater);
	map.process_one(&mut seeds, TransformType::ToLight);
	map.process_one(&mut seeds, TransformType::ToTemperature);
	map.process_one(&mut seeds, TransformType::ToHumidity);
	map.process_one(&mut seeds, TransformType::ToLocation);
	return seeds.into_iter().min().unwrap();
}

fn p2(map: &Maps) -> u64 {
	let mut seeds_iters = map.seeds.clone();
	let mut seeds: Vec<u64> = Vec::new();
	let mut i = 0;
	while i < seeds_iters.len() {
		let maxit = seeds_iters[i] + seeds_iters[i+1];
		let minit = seeds_iters[i];
		let mut min = std::u64::MAX;
		println!("Generado: {}", i/2);
		i += 2;
	}
	
	return seeds.into_iter().min().unwrap();
}

fn main() {
	let s = std::fs::read_to_string("./src/input.txt").expect("couldn't read the file for test");
	let map: Maps = Maps::new(s.as_str());
	let r1 = p1(&map);
	let r2 = p2(&map);
	println!("P1: {}\nP2: {}", r1, r2);
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_working() {
		let s = std::fs::read_to_string("./src/ex.txt").expect("couldn't read the file for test");
		let map: Maps = Maps::new(s.as_str());
		assert_eq!(p1(&map),35);
	}
	#[test]
	fn p2_working() {
		let s = std::fs::read_to_string("./src/ex.txt").expect("couldn't read the file for test");
		let map: Maps = Maps::new(s.as_str());
		assert_eq!(p2(&map),46);
	}
}

