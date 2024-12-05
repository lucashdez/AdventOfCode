use std::ptr::NonNull;
use std::rc::Rc;

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
	let read = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
	let mut sm = StateMachine::new();
	sm.add_state('c', false, true);
    println!("Hello, world!");
}
