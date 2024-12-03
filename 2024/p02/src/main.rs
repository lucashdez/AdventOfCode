use std::fs::read_to_string;

fn main() {
	let read = read_to_string("input/input.txt").expect("?");
	let v = read.split("\n")
		.filter(|d| *d != "")
		.map(|line| line.replace("\r", "").split(' ')
			 .filter(|d| *d != "")
			 .map(|n| n.parse::<i64>().unwrap())
			 .collect::<Vec<i64>>())
		.collect::<Vec<Vec<i64>>>();

	let mut sum = 0;
	for i in 0..v.len() {
		let all_increasing = v[i].windows(2).all(|pair| pair[0] < pair[1]);
		let all_decreasing = v[i].windows(2).all(|pair| pair[0] > pair[1]);

		if all_increasing || all_decreasing {
			let first = (v[i][1] - v[i][0]).abs();
			if first > 3 || first == 0 {continue};

			let second = (v[i][2] - v[i][1]).abs();
			if second > 3 || second == 0 {continue};

			let third  = (v[i][3] - v[i][2]).abs();
			if third > 3 || third == 0 {continue};

			let forth  = (v[i][4] - v[i][3]).abs();
			if forth > 3 || forth == 0 {continue};

			sum += 1;
		}
	}

    println!("P1: {sum}");
}
