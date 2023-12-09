struct Maps {
	seed_to_soil:            Vec<(u64, u64)>,
	seed_to_fertilizer:      Vec<(u64, u64)>,
	fertilizer_to_water:     Vec<(u64, u64)>,
	water_to_light:          Vec<(u64, u64)>,
	light_to_temperature:    Vec<(u64, u64)>,
	temperature_to_humidity: Vec<(u64, u64)>,
	humidity_to_location:    Vec<(u64, u64)>
}

impl Maps {
	fn new() {
	}
}

fn main() {
    println!("Hello, world!");
}

mod test {
	#[test]
	fn p1_working() {
		let s = std::fs::read_to_string("./src/ex.txt").expect("couldn't read the file for test");
		let v: Vec<&str> = s.split('\n').filter(|d| d != &"").collect();

		dbg!(v);

		assert_eq!(1, 0);
	}
}

