// mod avl;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Display;

type Tree<T> = Rc<RefCell<AVLNode<T>>>;
type AVLTree<T> = Option<Tree<T>>;

#[derive(Clone, Debug)]
struct AVLNode<T> {
    pub key: T,
    height: i32,
    left: AVLTree<T>,
    right: AVLTree<T>,
}

#[derive(Clone, Debug)]
struct AVL<T: Ord+Copy+Display> {
    // this is bad solution to the problem but
    // fixing this by adding a new "blank" node as the precursor
    root: AVLTree<T>
}

#[derive(Debug)]
enum RotationCase {
    Nil,
    LL,
    RR,
    LR,
    RL
}

impl<T: Ord+Copy+Display> AVLNode<T> {
    pub fn new(data: T) -> AVLTree<T> {
        Some(Rc::new(RefCell::new(Self {
            key: data,
            height: 1,
            left: None,
            right: None
        })))
    }

    fn height(node: AVLTree<T>) -> i32 {
        return match node {
            None => {
                0
            }
            Some(cur) => {
                cur.borrow().height
            }
        }
    }

    fn getMinVal(node: AVLTree<T>) -> T {
        let n_unwrap = node.unwrap();
        let n_bor = n_unwrap.borrow();
        return if n_bor.right.is_some() {
            Self::getMinVal(n_bor.right.clone())
        } else {
            n_bor.key
        }
    }

    fn getBalance(node: AVLTree<T>) -> i32 {
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
    fn insert(node: AVLTree<T>, data : T) -> AVLTree<T> {
        println!("Insertion called..");

        if node.is_none() { // no node exist thus insert one here
            println!("added node");
            return Self::new(data);
        }

        let rc_node = node.clone().unwrap();
        let bal;
        let rcase : RotationCase;
        {  // mutable block for changing height + bst insert
            let mut cur = rc_node.borrow_mut();

            if data < cur.key {
                cur.left = Self::insert(cur.left.clone(), data);
            } else if data > cur.key {
                cur.right = Self::insert(cur.right.clone(), data);
            } else {
                return node;
            }

            cur.height = 1 + Self::height(cur.left.clone()).max(Self::height(cur.right.clone()));
            println!("height {}", cur.height);

            bal = Self::height(cur.left.clone()) - Self::height(cur.right.clone());
            println!("balance {}", bal);

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

        println!("rcase {:?}", rcase);
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
                let tmp = Self::leftRotate(rc_node.borrow().right.clone().unwrap());
                rc_node.borrow_mut().right = tmp;
                Self::leftRotate(rc_node)
            }
            RotationCase::Nil => {
                node
            }
        }
    }

    fn rightRotate(node: Tree<T>) -> AVLTree<T> {
        println!("RR");
        let mut cur = node.borrow_mut(); // y
        let l = cur.left.clone(); // x
        let l_unwrap = l.clone().unwrap();
        let mut l_mut = l_unwrap.borrow_mut(); //x

        let lr = l_mut.right.clone(); //t2

        // rotate
        l_mut.right = Some(node.clone());
        cur.left = lr;

        // update height
        cur.height = 1 + Self::height(cur.left.clone()).max(Self::height(cur.right.clone()));
        l_mut.height = 1 + Self::height(l_mut.left.clone()).max(cur.height);

        return l;
    }

    fn leftRotate(node: Tree<T>) -> AVLTree<T> {
        println!("LR");
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

    fn delete(root: AVLTree<T>, data : T) -> AVLTree<T> {
        println!("Delete called");
        if root.is_none() {
            return root
        }

        let rc_node = root.clone().unwrap();
        let rcase : RotationCase;
        // delete -
        let mut rem_node: AVLTree<T> = None;
        // let mut rem_mut;
        {
            let mut cur = rc_node.borrow_mut();
            let rem_unwrap;
            let mut rem_mut;

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

            // let rem_unwrap = rem_node.clone().unwrap();
            // let mut rem_mut = rem_unwrap.borrow_mut();
            rem_mut.height = 1 + Self::height(rem_mut.left.clone()).max(Self::height(rem_mut.right.clone()));

            let bal = Self::height(rem_mut.left.clone()) - Self::height(rem_mut.right.clone());
            let rbal = Self::getBalance(rem_mut.right.clone());
            let lbal = Self::getBalance(rem_mut.left.clone());

            println!("balance {}", bal);

            if (bal > 1) && (lbal >= 0) {
                rcase = RotationCase::LL;
            } else if (bal < -1) && (rbal <= 0) {
                rcase = RotationCase::RR;
            } else if (bal > 1) && (lbal < 0) {
                rcase = RotationCase::LR;
            } else if (bal < -1) && (rbal > 0) {
                rcase = RotationCase::RL;
            } else {
                rcase = RotationCase::Nil;
            }
        }

        if rem_node.is_none() {
            return rem_node;
        }

        let rem_unwrap = rem_node.clone().unwrap();

        println!("rcase {:?}", rcase);
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
                let tmp = Self::leftRotate(rem_unwrap.borrow().right.clone().unwrap());
                rem_unwrap.borrow_mut().right = tmp;
                Self::leftRotate(rem_unwrap)
            }
            RotationCase::Nil => {
                rem_node
            }
        }
    }

    fn count(root: AVLTree<T>) -> i32 {
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
    fn inorder(root: AVLTree<T>) {
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

    fn isEmpty(root: AVLTree<T>) -> bool {
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
    fn prettyPrint(root: AVLTree<T>) -> String {
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
                Self::prettyPrintHelper(rc_sb.clone(), "", pointerLeft, n.left.clone(), n.right.is_some());
                Self::prettyPrintHelper(rc_sb.clone(), "", pointerRight, n.right.clone(), false);

                rc_sb.clone().borrow_mut().to_string()
            }
        }
    }

    fn prettyPrintHelper(rc_sb : Rc<RefCell<String>>, padding : &str, pointer : &str, node : AVLTree<T>, hasRight : bool) {
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

// blank node as precursor that has root on its right always
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
}
