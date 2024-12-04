use std::fs::read_to_string;

fn main() {
	let _read = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
	let read = read_to_string("input/input.txt").expect("?");
	let v = read.split("\n")
		.filter(|d| *d != "")
		.map(|line| line.replace("\r", "").split(' ')
			 .filter(|d| *d != "")
			 .map(|n| n.parse::<i64>().unwrap())
			 .collect::<Vec<i64>>())
		.collect::<Vec<Vec<i64>>>();

	let mut sum = 0;
	// P1
	for i in 0..v.len() {
		let all_increasing = v[i].windows(2).all(|pair| pair[0] < pair[1]);
		let all_decreasing = v[i].windows(2).all(|pair| pair[0] > pair[1]);
		if all_increasing {
			if v[i].windows(2).all(|pair| {
				let diff = pair[1] - pair[0];
				diff >= 1 && diff <= 3
			}) {
				sum += 1;
			}
		}

		if all_decreasing {

			if v[i].windows(2).all(|pair| {
				let diff = pair[0] - pair[1];
				diff >= 1 && diff <= 3
			}) {
				sum += 1;
			}
		}
	}


	//P2
	let mut sum2 = 0;
	for i in 0..v.len() {
		for j in 0..v[i].len() {
			let inc = v[i].clone()
				.iter()
				.enumerate()
				.filter(|(k, _)| j != *k)
				.map(|(_, val)| *val)
				.collect::<Vec<i64>>()
				.windows(2)
				.all(|p| (p[1] > p[0]) && (p[1] - p[0] > 0) && (p[1] - p[0] < 4));

			let dec = v[i].clone()
				.iter()
				.enumerate()
				.filter(|(k, _)| j != *k)
				.map(|(_, val)| *val)
				.collect::<Vec<i64>>()
				.windows(2)
				.all(|p| (p[0] > p[1]) && (p[0] - p[1] > 0) && (p[0] - p[1] < 4));

			if inc || dec {
				sum2 += 1;
				break;
			}
			
		}
	}
    println!("P1: {sum}");
	println!("P2: {sum2}");
}
