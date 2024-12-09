use std::{ops::Range, rc::Rc, collections::VecDeque};
use regex::*;

struct StateMachine {
	first: Rc<State>,
	last: Rc<State>,
}

impl StateMachine {
	fn add_state(&mut self, c: char, last: bool, autoref: bool) {
		if autoref {
			let new_edge = Edge::new(c, self.last.clone());
			Rc::get_mut(&mut self.last.clone()).unwrap().next.push(new_edge);
		} else {
			let newstate_ = Rc::new(State::new(last)); 
			let new_edge = Edge::new(c, newstate_.clone());
			Rc::get_mut(&mut self.last.clone()).unwrap().next.push(new_edge);
			self.last = newstate_.clone();
		}
	}
	fn new() -> Self {
		let mut s = StateMachine {
			first: Rc::new(State::new(false)),
			last: Rc::new(State::new(false)),
		};
		s.last = s.first.clone();
		Self {
			first: s.first,
			last: s.last,
		}
	}
}

struct Edge {
	consumes: char,
	to: Rc<State>,
}
impl Edge {
	fn new(consumes: char, to: Rc<State>) -> Self {
		Self {
			consumes,
			to
		}
	}
}

struct State {
	next: Vec<Edge>,
	acceptation: bool,
}

impl State {
	fn new(acceptation: bool) -> Self {
		State {
			next: Vec::new(),
			acceptation
		}
	}
}


fn main() {
	//let read: String = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
	let read = std::fs::read_to_string("input/input.txt").expect("Couldnt read file");
	let r = Regex::new(r"mul\((\d+),(\d+)\)").expect("Unexpected error");
	let mut valid: Vec<(i64, (i64, i64))> = Vec::new();
	for full in r.find_iter(&read) {
		let inrgx = Regex::new(r"\((\d+),(\d+)\)").unwrap();
		let (f, [x, y]) = inrgx.captures(full.as_str()).unwrap().extract();
		let start = full.range().start;
		let xp: i64 = x.parse().unwrap();
		let yp: i64 = y.parse().unwrap();
		let xl = x.len();
		let yl = y.len();
		if xl < 4 && xl > 0
			&& yl < 4 && yl > 0
		{
			valid.push((start as i64, (xp, yp)));
		}
	}

	let r2 = Regex::new(r"(?:mul\((?<f0>\d+),(?<f1>\d+)\))|(?<yes>do\(\))|(?<no>don't\(\))").unwrap();
	let mut valid2: Vec<(i64, i64)> = Vec::new();
	let mut active = true;
	for cap in r2.captures_iter(&read) {
		if cap.name("yes").is_some() {
			active = true;
		}
		if cap.name("no").is_some() {
			active = false;
		}

		if active && cap.name("f0").is_some() && cap.name("f1").is_some() {
			let f0 = cap.name("f0").unwrap().as_str();
			let f1 = cap.name("f1").unwrap().as_str();
			valid2.push((f0.parse().unwrap(), f1.parse().unwrap()))
		}
	}


	let result_p1: i64 = valid.iter().map(|x| x.1.0 * x.1.1).sum();
	let result_p2: i64 = valid2.iter().map(|x| x.0 * x.1).sum();

	println!("Part 1: {result_p1}\nPart 2: {result_p2}");
}
