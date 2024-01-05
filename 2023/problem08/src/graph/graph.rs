use std::rc::Rc;
use core::cell::RefCell;

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
			Direction::Left => self.left,
			Direction::Right => self.right
		}
	}
	pub fn set(&mut self, node: Node, d: Direction) {
		let new_node = Rc::new(RefCell::new(node));
		match d {
			Direction::Left => self.left = Some(Rc::clone(&new_node)),
			Direction::Right => self.right = Some(Rc::clone(&new_node))
		}
	}
}

pub struct Graph {
	gnome: Box<Option<Node>>
}
