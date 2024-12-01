use std::fs::read_to_string;
use std::collections::HashMap;


fn main() {
    println!("Hello, world!");

	let p1 = read_to_string("input/p1.txt").expect("WTF");
	let mut v1: Vec<i64> = Vec::new();
	let mut v2: Vec<i64> = Vec::new();
	p1.split("\n").filter(|x| (*x) != "").for_each(|line|  {
		let parts = line.split_once(' ').unwrap();
		v1.push(parts.0.trim().parse::<i64>().unwrap());
		v2.push(parts.1.trim().parse::<i64>().unwrap());
	});

	v1.sort();
	v2.sort();	

	if v1.len() != v2.len() {
		panic!("How did we get here?");
	}

	let mut accum = 0;
	for i in 0..v1.len() {
		accum += (v2[i] - v1[i]).abs();
	}

	let mut hm: HashMap<i64, u64> = HashMap::new();
	for i in 0..v2.len() {
		if !hm.contains_key(&v2[i]) {
			hm.insert(v2[i], 1);
		} else {
			let v = hm.get_mut(&v2[i]).unwrap();
			*v += 1;
		}
	}
	let mut sim_score = 0;
	for i in 0..v1.len() {
		match hm.get(&v1[i]) {
			Some(x) => sim_score += v1[i] * ((*x) as i64),
			None => (),
		}
	}

	println!("P1: {accum}");
	println!("P2: {sim_score}");
}
