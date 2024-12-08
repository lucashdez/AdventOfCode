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
	// let read: String = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
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

	let do_reg = Regex::new(r"do\(\)").unwrap();

	let mut dos: Vec<i64> = do_reg.find_iter(&read).map(|cap| cap.range().start as i64).collect::<Vec<i64>>();
	let don_reg = Regex::new(r"don't\(\)").unwrap();
	let mut dons: Vec<i64> = don_reg.find_iter(&read).map(|cap| cap.range().start as i64).collect::<Vec<i64>>();

	dos.sort();
	dons.sort();

	let mut dos_q = VecDeque::from_iter(dos.iter());
	dos_q.push_front(&0);
	let mut dons_q = VecDeque::from_iter(dons.iter());
	let mut ranges: Vec<(Range<i64>, bool)> = Vec::new();
	let mut last = 0; 
	let mut last_mode = true;


	while !dons_q.is_empty() && !dos_q.is_empty() {
		if dons_q.front().is_some() && dos_q.front().is_some() && dons_q.front() < dos_q.front() {
			ranges.push((last..*dons_q.pop_front().unwrap(), last_mode))
		}
	}


	let max_i = valid[valid.len()-1].0;
	if ranges[ranges.len()-1].1 {
		ranges.push((ranges[ranges.len()-1].0.end..max_i+1, true))
	} else {
		ranges.push((ranges[ranges.len()-1].0.end..max_i+1, false))
	}


	println!("{}, {}", dos.len(), dons.len());
	let result_p1: i64 = valid.iter().map(|x| x.1.0 * x.1.1).sum();
	let result_p2: i64 = valid.iter().filter(|val| ranges.iter().any(|x| x.1 && x.0.contains(&val.0))).map(|x| x.1.0 * x.1.1).sum();

	println!("Part 1: {result_p1}\nPart 2: {result_p2}");
}
