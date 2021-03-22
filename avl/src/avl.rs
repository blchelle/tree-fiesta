// mod avl;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Display;

type Tree<T> = Rc<RefCell<AVLNode<T>>>;
type AVLTree<T> = Option<Tree<T>>;

#[derive(Clone, Debug)]
pub struct AVLNode<T> { // actual node
    pub key: T,
    height: i32,
    left: AVLTree<T>,
    right: AVLTree<T>,
}

// tree object that allows calls to self
// easier usage for user and abstracts the nodes away from tree
#[derive(Clone, Debug)]
pub struct AVL<T: Ord+Copy+Display> {
    root: AVLTree<T>
}

// a enum that holds the rotation cases for clarity
#[derive(Debug)]
enum RotationCase {
    Nil,
    LL,
    RR,
    LR,
    RL
}

impl<T: Ord+Copy+Display> AVLNode<T> {
    // creates a new avl node with data
    pub fn new(data: T) -> AVLTree<T> {
        Some(Rc::new(RefCell::new(Self {
            key: data,
            height: 1,
            left: None,
            right: None
        })))
    }

    // gets height of tree, 0 if node doesn't exist
    pub fn height(node: AVLTree<T>) -> i32 {
        return match node {
            None => {
                0
            }
            Some(cur) => {
                cur.borrow().height
            }
        }
    }

    // gets the min key of tree rooted at node given
    pub fn getMinVal(node: AVLTree<T>) -> T {
        let n_unwrap = node.unwrap();
        let n_bor = n_unwrap.borrow();
        return if n_bor.right.is_some() {
            Self::getMinVal(n_bor.right.clone())
        } else {
            n_bor.key
        }
    }

    // calculates the avl balance algo. returns the new root
    pub fn getBalance(node: AVLTree<T>) -> i32 {
        match node {
            None => {
                0
            }
            Some(n) => {
                let n_b = n.borrow();
                Self::height(n_b.left.clone()) - Self::height(n_b.right.clone())
            }
        }
    }

    // insert treating self as the root
    pub fn insert(node: AVLTree<T>, data : T) -> AVLTree<T> {
        if node.is_none() { // no node exist thus insert one here
            return Self::new(data);
        }

        let rc_node = node.clone().unwrap();
        let bal;
        let rcase : RotationCase;
        {  // mutable block for changing height + bst insert
            let mut cur = rc_node.borrow_mut();

            // recursive stanadard binary tree insertion
            if data < cur.key {
                cur.left = Self::insert(cur.left.clone(), data);
            } else if data > cur.key {
                cur.right = Self::insert(cur.right.clone(), data);
            } else {
                return node;
            }

            // calculate properties
            cur.height = 1 + Self::height(cur.left.clone()).max(Self::height(cur.right.clone()));
            bal = Self::height(cur.left.clone()) - Self::height(cur.right.clone());

            let l_key;
            let r_key;
            match cur.left.clone() {
                None => {
                    l_key = data;
                }
                Some(l_node) => {
                    l_key = l_node.borrow().key
                }
            }
            match cur.right.clone() {
                None => {
                    r_key = data;
                }
                Some(r_node) => {
                    r_key = r_node.borrow().key
                }
            }

            // compute rotation cases
            if (bal > 1) && (data < l_key) {
                rcase = RotationCase::LL;
            } else if (bal < -1) && (data > r_key) {
                rcase = RotationCase::RR;
            } else if (bal > 1) && (data > l_key) {
                rcase = RotationCase::LR;
            } else if (bal < -1) && (data < r_key) {
                rcase = RotationCase::RL;
            } else {
                rcase = RotationCase::Nil;
            }
        }

        // return the node after rotation cases run
        return match rcase {
            RotationCase::LL => {
                Self::rightRotate(rc_node)
            }
            RotationCase::RR => {
                Self::leftRotate(rc_node)
            }
            RotationCase::LR => {
                let tmp = Self::leftRotate(rc_node.borrow().left.clone().unwrap());
                rc_node.borrow_mut().left = tmp;
                Self::rightRotate(rc_node)
            }
            RotationCase::RL => {
                let tmp = Self::rightRotate(rc_node.borrow().right.clone().unwrap());
                rc_node.borrow_mut().right = tmp;
                Self::leftRotate(rc_node)
            }
            RotationCase::Nil => {
                node
            }
        }
    }

    // algo for right rotations
    pub fn rightRotate(node: Tree<T>) -> AVLTree<T> {
        let mut cur = node.borrow_mut();
        let l = cur.left.clone();
        let l_unwrap = l.clone().unwrap();
        let mut l_mut = l_unwrap.borrow_mut();

        let lr = l_mut.right.clone();

        // rotate
        l_mut.right = Some(node.clone());
        cur.left = lr;

        // update height
        cur.height = 1 + Self::height(cur.left.clone()).max(Self::height(cur.right.clone()));
        l_mut.height = 1 + Self::height(l_mut.left.clone()).max(cur.height);

        return l;
    }

    // algo for left rotatons
    pub fn leftRotate(node: Tree<T>) -> AVLTree<T> {
        let mut cur = node.borrow_mut(); // x
        let r = cur.right.clone(); // x .right (y)
        let r_unwrap = r.clone().unwrap();
        let mut r_mut = r_unwrap.borrow_mut(); // y

        let rl = r_mut.left.clone(); // t2

        // rotate
        r_mut.left = Some(node.clone());
        cur.right = rl;

        // update height
        cur.height = 1 + Self::height(cur.left.clone()).max(Self::height(cur.right.clone()));
        r_mut.height = 1 + cur.height.max(Self::height(r_mut.right.clone()));

        return r;
    }

    // returns bool if node exists
    pub fn search(root: AVLTree<T>, data : T) -> bool {
        return match root {
            None => {
               false
            },
            Some(node) => {
                let n = node.borrow();
                if data < n.key {
                    return Self::search(n.left.clone(), data);
                } else if data > n.key {
                    return Self::search(n.right.clone(), data);
                } else {
                    true
                }
            }
        }
    }

    // deletes starting at root returns new root
    pub fn delete(root: AVLTree<T>, data : T) -> AVLTree<T> {
        if root.is_none() { // no deletion cases
            return root
        }

        let rc_node = root.clone().unwrap();
        let rcase : RotationCase;
        let mut rem_node: AVLTree<T> = None;
        {   // mutability block
            let mut cur = rc_node.borrow_mut();
            let rem_unwrap;
            let mut rem_mut;

            // standard bst recurse deletion
            if data < cur.key {
                cur.left = Self::delete(cur.left.clone(), data);
                rem_node = root;
                rem_mut = cur;
            } else if data > cur.key {
                cur.right = Self::delete(cur.right.clone(), data);
                rem_node = root;
                rem_mut = cur;
            } else { // delete here
                if cur.left.is_none() && cur.right.is_none() { // no child
                    return None;
                } else if cur.left.is_some() && cur.right.is_some() {
                    // get inorder, copy keys, delete lowest
                    let tmp = Self::getMinVal(cur.right.clone());
                    cur.key = tmp;
                    cur.right = Self::delete(cur.right.clone(), tmp);
                    rem_node = root.clone();
                    rem_mut = cur;
                } else if cur.left.is_some() {
                    rem_node = cur.left.clone();
                    rem_unwrap = rem_node.clone().unwrap();
                    rem_mut = rem_unwrap.borrow_mut();
                } else if cur.right.is_some() {
                    rem_node = cur.right.clone();
                    rem_unwrap = rem_node.clone().unwrap();
                    rem_mut = rem_unwrap.borrow_mut();
                } else {
                    panic!("This cond doesn't exist");
                }
            }

            // rem_node shouldn't be none but if it is just return no root
            if rem_node.is_none() {
                println!("the impossible happened");
                return rem_node;
            }

            rem_mut.height = 1 + Self::height(rem_mut.left.clone()).max(Self::height(rem_mut.right.clone()));
            // get balancing conditions
            let bal = Self::height(rem_mut.left.clone()) - Self::height(rem_mut.right.clone());
            let rbal = Self::getBalance(rem_mut.right.clone());
            let lbal = Self::getBalance(rem_mut.left.clone());

            if (bal > 1) && (lbal >= 0) {   // ll
                rcase = RotationCase::LL;
            } else if (bal < -1) && (rbal <= 0) { // rr
                rcase = RotationCase::RR;
            } else if (bal > 1) && (lbal < 0) {
                rcase = RotationCase::LR;
            } else if (bal < -1) && (rbal > 0) {
                rcase = RotationCase::RL;
            } else {
                rcase = RotationCase::Nil;
            }
        }

        let rem_unwrap = rem_node.clone().unwrap();

        // return new root after rotations
        return match rcase {
            RotationCase::LL => {
                Self::rightRotate(rem_unwrap)
            }
            RotationCase::RR => {
                Self::leftRotate(rem_unwrap)
            }
            RotationCase::LR => {
                let tmp = Self::leftRotate(rem_unwrap.borrow().left.clone().unwrap());
                rem_unwrap.borrow_mut().left = tmp;
                Self::rightRotate(rem_unwrap)
            }
            RotationCase::RL => {
                let tmp = Self::rightRotate(rem_unwrap.borrow().right.clone().unwrap());
                rem_unwrap.borrow_mut().right = tmp;
                Self::leftRotate(rem_unwrap)
            }
            RotationCase::Nil => {
                rem_node
            }
        }
    }

    // counts nodes rooted at root
    pub fn count(root: AVLTree<T>) -> i32 {
        return match root {
            None => {
                0
            }
            Some(node) => {
                let n = node.borrow();
                return 1 + Self::count(n.left.clone()) + Self::count(n.right.clone());
            }
        }
    }

    // print inorder traversal
    pub fn inorder(root: AVLTree<T>) {
        match root {
            None => {}
            Some(node) => {
                let n = node.borrow();
                Self::inorder(n.left.clone());
                print!(" {} ", n.key);
                Self::inorder(n.right.clone());
            }
        }
    }

    // computes if empty
    pub fn isEmpty(root: AVLTree<T>) -> bool {
        match root {
            None => {
                true
            }
            Some(_) => {
                false
            }
        }
    }

    /**
        algorithmic idea drawn from https://www.baeldung.com/java-print-binary-tree-diagram
    */
    pub fn prettyPrint(root: AVLTree<T>) -> String {
        return match root {
            None => {
                "".to_string()
            }
            Some(node) => {
                let rc_sb = Rc::from(RefCell::from(String::from("")));
                let n = node.borrow();
                {
                    let mut sb = rc_sb.borrow_mut();
                    sb.push_str(n.key.clone().to_string().as_str());
                }

                let pointerRight = "└──";
                // String pointerLeft = (root.getRight() != null) ? "├──" : "└──";
                let pointerLeft;
                if n.right.is_none() {
                    pointerLeft = "└──";
                } else {
                    pointerLeft = "├──";
                }
                // let pointLeft =
                Self::prettyPrintHelper(rc_sb.clone(), "", pointerRight, n.right.clone(), false);
                Self::prettyPrintHelper(rc_sb.clone(), "", pointerLeft, n.left.clone(), n.right.is_some());
                // Self::prettyPrintHelper(rc_sb.clone(), "", pointerRight, n.right.clone(), false);

                rc_sb.clone().borrow_mut().to_string()
            }
        }
    }

    // helpers the pretty printer draw recursively
    pub fn prettyPrintHelper(rc_sb : Rc<RefCell<String>>, padding : &str, pointer : &str, node : AVLTree<T>, hasRight : bool) {
        match node {
            None => {}
            Some(cur) => {
                let n = cur.borrow();
                {
                    let mut sb = rc_sb.borrow_mut();
                    sb.push_str("\n");
                    sb.push_str(padding);
                    sb.push_str(pointer);
                    sb.push_str(n.key.clone().to_string().as_str());
                }

                let mut psb = String::from(padding);
                if hasRight {
                    psb.push_str("│  ");
                } else {
                    psb.push_str("   ");
                }

                let bothPad = String::from(psb);
                let pointerRight = "└──";
                // String pointerLeft = (root.getRight() != null) ? "├──" : "└──";
                let pointerLeft;
                if n.right.is_none() {
                    pointerLeft = "└──";
                } else {
                    pointerLeft = "├──";
                }

                Self::prettyPrintHelper(rc_sb.clone(), &*bothPad, pointerLeft, n.left.clone(), n.right.is_some());
                Self::prettyPrintHelper(rc_sb.clone(), &*bothPad, pointerRight, n.right.clone(), false);
            }
        }
    }
}

// implementation that abstracts the details of the nodes away from the user
impl<T: Ord+Copy+Display> AVL<T> {
    pub fn new() -> Self {
        Self {root: None}
    }

    pub fn isEmpty(&mut self) -> bool {
        // let node = self.root
        AVLNode::isEmpty(self.root.clone())
    }

    pub fn insert(&mut self, data : T) {
        self.root = AVLNode::insert(self.root.clone(), data);
    }

    pub fn delete(&mut self, data : T) {
        self.root = AVLNode::delete(self.root.clone(), data);
    }

    pub fn count(&self) -> i32 {
        AVLNode::count(self.root.clone())
    }

    pub fn height(&self) -> i32 {
        AVLNode::height(self.root.clone())
    }

    pub fn inorder(&self) {
        AVLNode::inorder(self.root.clone());
        println!();
    }

    pub fn print(&self) -> String {
        AVLNode::prettyPrint(self.root.clone())
    }

    pub fn search(&self, data : T) -> bool {
        AVLNode::search(self.root.clone(), data)
    }
}
