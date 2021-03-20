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
        {  // mutable block for changing height
            let mut cur = rc_node.borrow_mut();

            if data < cur.key {
                println!("-> l");
                cur.left = Self::insert(cur.left.clone(), data);
            } else if data > cur.key {
                println!("-> r");
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
        println!("updating heights");
        cur.height = 1 + Self::height(cur.left.clone()).max(Self::height(cur.right.clone()));
        println!("updating heights");
        r_mut.height = 1 + cur.height.max(Self::height(r_mut.right.clone()));

        return r;
    }
}


fn main() {
    println!("Hello, world!");
    let root = AVLNode::new(1);
    AVLNode::insert(root.clone(), 2);
    println!("added 2");
    AVLNode::insert(root.clone(), 3);
    println!("added 3");
    AVLNode::insert(root.clone(), 4);
    AVLNode::insert(root.clone(), 5);
    AVLNode::insert(root.clone(), 6);
    AVLNode::insert(root.clone(), 7);
    AVLNode::insert(root.clone(), 8);


    // AVLNode::insert(root, 1);
}
