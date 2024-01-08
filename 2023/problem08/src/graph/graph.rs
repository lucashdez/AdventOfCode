use std::rc::Rc;
use core::cell::RefCell;
use std::collections::HashMap;

type Link<T> = Option<Rc<RefCell<T>>>; 

pub enum Direction {
	Left,
	Right
}

pub struct Node {
	val: String,
	left: Link<Node>,
	right: Link<Node>
}

impl std::fmt::Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		let the_none = Rc::new(RefCell::new(Node::new("None".to_string())));
		write!(f, "name: {} (l: {}, r: {})",
			   self.val,
			   self.left.clone().unwrap_or(the_none.clone()).borrow().val,
			   self.right.clone().unwrap_or(the_none).borrow().val
		)
	}
}

impl Node {
	pub fn new(name: String) -> Self {
		Node {val: name,
			 left: None,
			  right: None 
		}
	}
	pub fn name(&self) -> &String {
		&self.val
	}
	pub fn get(&self, d: Direction) -> Link<Node> {
		match d {
			Direction::Left => self.left.clone(),
			Direction::Right => self.right.clone()
		}
	}
	pub fn set(&mut self, node: Link<Node>, d: Direction) {
		match d {
			Direction::Left => self.left = node,
			Direction::Right => self.right = node
		}
	}
}

pub struct GraphTraverser {
	gnome: Link<Node>,
	node_count: usize,
	graph: HashMap<String, Link<Node>>
}

impl GraphTraverser {
	// {{{ new (rules: &str)
	pub fn new(rules: &str) -> Self {
		let mut first_node: Link<Node> = None;
		let mut all_rules: HashMap<String, Link<Node>> = HashMap::<String, Link<Node>>::new();
		rules
			.split('\n')
			.filter(|f| f != &"")
			.for_each(|rule| {
				let (base, leaf) = rule.split_once(" = ").unwrap();
				let mut actual_node: Link<Node> = None; 
				if all_rules.contains_key(&base.to_string()) {
					actual_node = all_rules.get_mut(&base.to_string()).unwrap().clone();
				} else {
					actual_node = Some(Rc::new(RefCell::new(Node::new(base.to_string()))));
					all_rules.insert(base.to_string(), actual_node.clone());
				}
				let (mut left, mut right) = leaf.split_once(", ").expect("???");
				left = &left[1..];
				right = &right[..right.len()-1];
				if let Some(node) = actual_node {
					let mut left_node: Link<Node> = None;
					let mut right_node: Link<Node> = None;

					all_rules.entry(left.to_string())
						.and_modify(|v| left_node = v.clone())
						.or_insert_with(|| {
							left_node = Some(Rc::new(RefCell::new(Node::new(left.to_string()))));
							return left_node.clone();
						});

					all_rules.entry(right.to_string())
						.and_modify(|v| right_node = v.clone())
						.or_insert_with(|| {
							right_node = Some(Rc::new(RefCell::new(Node::new(right.to_string()))));
							return right_node.clone();
						});

					node.borrow_mut().set(left_node, Direction::Left);
					node.borrow_mut().set(right_node, Direction::Right);
				}
			});
		first_node = all_rules.get("AAA")
			.expect("Impossible").clone();
		GraphTraverser {
			gnome: first_node,
			node_count: all_rules.len(),
			graph: all_rules
		}
	}
	// }}}

	pub fn traverse(&mut self, path: &str) -> Option<usize> {
		let max_iterations : usize = usize::MAX;
		let mut it: usize = 0; 
		let mut s_pos: usize = 0;
		let movements = path.chars().filter(|f| f != &' ').collect::<Vec<char>>();
		while it < max_iterations {
			if s_pos == movements.len() { s_pos = 0; }
			let mut next_node: Link<Node> = None;
			if movements[s_pos] == 'L' {
				next_node = self.gnome.clone().unwrap().borrow().left.clone();
			}
			if movements[s_pos] == 'R' {
				next_node = self.gnome.clone().unwrap().borrow().right.clone();
			}
			self.gnome = next_node;
			s_pos += 1;
			it += 1;
			if self.gnome.clone().unwrap().borrow().val == "ZZZ" {
				return Some(it);
			}
		}
		None
	}
}

impl std::fmt::Display for GraphTraverser {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "actual: {:?}", self.gnome)
	}
}
