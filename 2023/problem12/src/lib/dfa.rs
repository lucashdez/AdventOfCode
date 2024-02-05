/// Something
struct Transition {
	symbol: char,
	next: Node
}

impl Transition {
	pub fn new(s: char) -> Self {
		Transition {
			symbol: s,
			next: Node::new()
		}
	}
}

struct Node {
	transitions: Vec<Transition>
}

impl Node {
	pub fn new() -> Self{
		Node {
			transitions: Vec::new()
		}
	}
}

struct DFA {
	head: Node
}
