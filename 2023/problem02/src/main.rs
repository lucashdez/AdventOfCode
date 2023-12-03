use regex::Regex;
struct GameLimits {
	reds: u32,
	greens: u32,
	blues: u32,
}

fn p1(input: &[&str], limits: GameLimits) -> u32 {
	let mut result = 0;
	input.into_iter().for_each(|s| {
		let game_rg  = Regex::new(r"Game (?<gameid>\d+): (?<rest>.*)").unwrap();
		let green_rg = Regex::new(r"(\d+) green").unwrap();
		let blue_rg  = Regex::new(r"(\d+) blue").unwrap();
		let red_rg   = Regex::new(r"(\d+) red").unwrap();
		if let Some(game) = game_rg.captures(&s) {
			let mut game_valid = true;
			let game_id_s = &game["gameid"];
			let game_id: u32 = game_id_s.parse().unwrap();
			let rest_text = &game["rest"];
			for capture in red_rg.find_iter(&rest_text) {
				let num: u32 = capture.as_str().split(' ').collect::<Vec<&str>>()[0].parse().unwrap();
				if game_valid {
					game_valid = num <= limits.reds;
				}
			}
			for capture in green_rg.find_iter(&rest_text) {
				let num: u32 = capture.as_str().split(' ').collect::<Vec<&str>>()[0].parse().unwrap();
				if game_valid {
					game_valid = num <= limits.greens;
				}
			}

			for capture in blue_rg.find_iter(&rest_text) {
				let num: u32 = capture.as_str().split(' ').collect::<Vec<&str>>()[0].parse().unwrap();
				if game_valid {
					game_valid = num <= limits.blues;
				}
			}

			if game_valid {
				result += game_id;
			}
		}
	});
	return result;
}

fn p2(input: &[&str]) -> u32 {
	let mut result = 0;
	input.into_iter().for_each(|s| {
		let game_rg  = Regex::new(r"Game (?<gameid>\d+): (?<rest>.*)").unwrap();
		let green_rg = Regex::new(r"(\d+) green").unwrap();
		let blue_rg  = Regex::new(r"(\d+) blue").unwrap();
		let red_rg   = Regex::new(r"(\d+) red").unwrap();
		if let Some(game) = game_rg.captures(&s) {
			let mut game_valid = true;
			let game_id_s = &game["gameid"];
			let game_id: u32 = game_id_s.parse().unwrap();
			let rest_text = &game["rest"];
			let mut max_reds = 0;
			let mut max_greens = 0;
			let mut max_blues = 0;
			
				
			for capture in red_rg.find_iter(&rest_text) {
				let num: u32 = capture.as_str().split(' ').collect::<Vec<&str>>()[0].parse().unwrap();
				max_reds = if num > max_reds {num} else {max_reds};
			}
			for capture in green_rg.find_iter(&rest_text) {
				let num: u32 = capture.as_str().split(' ').collect::<Vec<&str>>()[0].parse().unwrap();
				max_greens = if num > max_greens {num} else {max_greens};
			}

			for capture in blue_rg.find_iter(&rest_text) {
				let num: u32 = capture.as_str().split(' ').collect::<Vec<&str>>()[0].parse().unwrap();
				max_blues = if num > max_blues {num} else {max_blues};
			}
			result += max_reds * max_greens * max_blues;
		}
	});
	return result;
}

fn main() {
	let input = std::fs::read_to_string("./src/input.txt").expect("Couldn't open the file"); 
	let input_mod: Vec<&str> = input.split('\n').collect();
	let limits: GameLimits = GameLimits {reds: 12, greens: 13, blues:14};
	let r1 = p1(&input_mod, limits);
	let r2 = p2(&input_mod);
	println!("P1: {}\nP2: {}", r1, r2);
}


#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_working() {
		let limits: GameLimits = GameLimits {reds: 10, greens: 10, blues:10};
		let input_test = ["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
			"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
			"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
			"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
		 ];

		assert_eq!(p1(&input_test, limits), 8);
	}

	#[test]
	fn p2_working() {
		let input_test = ["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
			"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
			"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
			"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
		 ];
		
		assert_eq!(p2(&input_test), 2286);
	}
	
}
