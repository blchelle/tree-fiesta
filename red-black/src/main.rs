// Define the types
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
enum NodeColor {
	Red,
	Black,
}

/**
 * Type of a child node
 */
type Child = Option<Rc<RefCell<Node>>>;

/**
 * Type of a parent node.
 * The Rc here is weak to break any reference cycles
 */
type Parent = Option<Weak<RefCell<Node>>>;

/**
 * A representation of a node in a RedBlackTree
 */
#[derive(Debug)]
struct Node {
	value: i32,
	color: NodeColor,
	parent: Parent,
	left: Child,
	right: Child,
}

/**
 * A wrapper around the root node of the RBT
 */
#[derive(Debug)]
pub struct RBT {
	root: Child,
}

impl RBT {
	fn new(value: i32) -> Self {
		let root = Node {
			value: value,
			color: NodeColor::Black,
			parent: None,
			left: None,
			right: None,
		};

		RBT {
			root: Some(Rc::new(RefCell::new(root))),
		}
	}

	fn right_rotation(node: Child) {
		// If the rotating-node is None then something weird is happening
		if let None = node {
			println!("Rotation node in 'right_rotation' is None...");
			return;
		}

		let node = node.unwrap();

		// Step 1. Gets a reference to the left child of node
		// Note that if we're performing a right-rotation, this shouldn't be None
		let left_child = match node.borrow().left {
			None => panic!("Right rotation: this should never be None"),
			Some(ref node_left) => Rc::clone(node_left),
		};

		// Step 2. Get a reference to the left-right child
		// This can definitely be None, so that case needs to be handled
		let left_right_child = match left_child.borrow().right {
			None => None,
			Some(ref node_left_right) => Some(Rc::clone(node_left_right)),
		};

		// Step 3. Set the right child of the left child to be node
		left_child.borrow_mut().right = Some(Rc::clone(&node));

		// Step 3.5 Get a reference to the parent of the input node
		let parent = match node.borrow().parent {
			None => None,
			Some(ref node_parent) => match node_parent.upgrade() {
				None => None,
				Some(ref node_parent) => Some(Rc::clone(node_parent)),
			},
		};

		// Step 4. Set the parent of the left child to be the parent of node
		left_child.borrow_mut().parent = match parent {
			None => None,
			Some(ref node_parent) => Some(Rc::downgrade(node_parent)),
		};

		// Step 5. Set the parent of node to be left child
		node.borrow_mut().parent = Some(Rc::downgrade(&left_child));

		// Step 6. Set the left child of node to be the left-right child

		node.borrow_mut().left = match left_right_child {
			None => None,
			Some(ref node_left_right) => Some(Rc::clone(node_left_right)),
		};

		// Step 7. Set the parent of left-right child to be node
		match left_right_child {
			None => {}
			Some(ref node_left_right) => {
				node_left_right.borrow_mut().parent = Some(Rc::downgrade(&node))
			}
		}
	}

	fn left_rotation(node: Child) {
		// If the rotating-node is None then something weird is happening
		if let None = node {
			println!("Rotation node in 'right_rotation' is None...");
			return;
		}

		let node = node.unwrap();

		// Step 1. Gets a reference to the right child of node
		// Note that if we're performing a left-rotation, this shouldn't be None
		let right_child = match node.borrow().right {
			None => panic!("Left rotation: this should never be None"),
			Some(ref node_right) => Rc::clone(node_right),
		};

		// Step 2. Get a reference to the right-left child
		// This can definitely be None, so that case needs to be handled
		let right_left_child = match right_child.borrow().left {
			None => None,
			Some(ref node_right_left) => Some(Rc::clone(node_right_left)),
		};

		// Step 3. Set the left child of the right child to be node
		right_child.borrow_mut().left = Some(Rc::clone(&node));

		// Step 3.5 Get a reference to the parent of the input node
		let parent = match node.borrow().parent {
			None => None,
			Some(ref node_parent) => match node_parent.upgrade() {
				None => None,
				Some(ref node_parent) => Some(Rc::clone(node_parent)),
			},
		};

		// Step 4. Set the parent of the right child to be the parent of node
		right_child.borrow_mut().parent = match parent {
			None => None,
			Some(ref node_parent) => Some(Rc::downgrade(node_parent)),
		};

		// Step 5. Set the parent of node to be right child
		node.borrow_mut().parent = Some(Rc::downgrade(&right_child));

		// Step 6. Set the right child of node to be the right-left child
		node.borrow_mut().right = match right_left_child {
			None => None,
			Some(ref node_right_left) => Some(Rc::clone(node_right_left)),
		};

		// Step 7. Set the parent of right-left child to be node
		match right_left_child {
			None => {}
			Some(ref node_right_left) => {
				node_right_left.borrow_mut().parent = Some(Rc::downgrade(&node))
			}
		}
	}
}

impl Node {
	/**
	 * Flips the color of the node
	 * Red --> Black
	 * Black --> Red
	 */
	fn recolor(&mut self) {
		self.color = match self.color {
			NodeColor::Red => NodeColor::Black,
			NodeColor::Black => NodeColor::Red,
		}
	}
}

fn main() {
	let root = RBT::new(7);

	println!("{:?}", root);
}
