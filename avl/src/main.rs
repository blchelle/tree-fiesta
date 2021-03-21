// mod avl;
use std::cell::RefCell;
use std::rc::Rc;

type Tree<T> = Rc<RefCell<AVLNode<T>>>;
type AVLTree<T> = Option<Tree<T>>;

#[derive(Clone)]
struct AVLNode<T> {
    pub key: T,
    height: i32,
    left: AVLTree<T>,
    right: AVLTree<T>,
}

#[derive(Debug)]
enum RotationCase {
    Nil,
    LL,
    RR,
    LR,
    RL
}

impl<T: Ord+Copy> AVLNode<T> {
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
        if n_bor.right.is_some() {
            return Self::getMinVal(n_bor.right.clone());
        } else {
            return n_bor.key;
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
                println!("cond1");
                cur.left = Self::delete(cur.left.clone(), data);
                rem_node = root;
                rem_mut = cur;
            } else if data > cur.key {
                println!("cond 2");
                cur.right = Self::delete(cur.right.clone(), data);
                rem_node = root;
                rem_mut = cur;
            } else { // delete here
                println!("found");
                if cur.left.is_none() && cur.right.is_none() { // no child
                    println!("is none");
                    return None;
                } else if cur.left.is_some() && cur.right.is_some() {
                    println!("has some");
                    // get inorder, copy keys, delete lowest
                    let tmp = Self::getMinVal(cur.right.clone());
                    cur.key = tmp;
                    cur.right = Self::delete(cur.right.clone(), tmp);
                    rem_node = root.clone();
                } else if cur.left.is_some() {
                    rem_node = cur.left.clone();
                } else if cur.right.is_some() {
                    rem_node = cur.right.clone();
                }

                rem_unwrap = rem_node.clone().unwrap();
                rem_mut = rem_unwrap.borrow_mut();
            }

            println!("chillin");
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


}


fn main() {
    println!("Hello, world!");
    let mut root = AVLNode::new(1);
    root = AVLNode::insert(root.clone(), 2);
    root = AVLNode::insert(root.clone(), 3);
    root = AVLNode::insert(root.clone(), 4);
    root = AVLNode::insert(root.clone(), 5);
    root = AVLNode::insert(root.clone(), 6);
    root = AVLNode::insert(root.clone(), 7);
    root = AVLNode::insert(root.clone(), 8);
    root = AVLNode::delete(root.clone(), 8);
    root = AVLNode::delete(root.clone(), 7);
    root = AVLNode::delete(root.clone(), 6);
    root = AVLNode::delete(root.clone(), 5);
    root = AVLNode::delete(root.clone(), 4);
    root = AVLNode::delete(root.clone(), 3);
    root = AVLNode::delete(root.clone(), 2);
    root = AVLNode::delete(root.clone(), 1);

    if root.is_none() {
        println!("nice");
    }

    // AVLNode::insert(root, 1);
}
