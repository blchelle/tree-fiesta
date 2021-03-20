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
            println!("Node placed...");
            return Self::new(data);
        }

        let rc_node = node.clone().unwrap();
        let bal;
        let rcase : RotationCase;
        {  // mutable block for changing height
            let mut cur = rc_node.borrow_mut();

            if cur.key < data {
                cur.left = Self::insert(cur.left.clone(), data);
            } else if cur.key > data {
                cur.right = Self::insert(cur.right.clone(), data);
            } else {
                return node;
            }

            cur.height += Self::height(cur.left.clone()).max(Self::height(cur.right.clone()));
            println!("height {}", cur.height);

            bal = Self::height(cur.left.clone()) - Self::height(cur.right.clone());
            println!("balance {}", bal);

            // ll = 0
            // rr = 1
            // lr = 2
            // rr = 3
            let l = cur.left.clone()?;
            let l_node = l.borrow();
            let r = cur.right.clone()?;
            let r_node = r.borrow();

            if (bal > 1) && (data < l_node.key) {
                rcase = RotationCase::LL;
            } else if (bal < -1) && (data > r_node.key) {
                rcase = RotationCase::RR;
            } else if (bal > 1) && (data > l_node.key) {
                rcase = RotationCase::LR;
            } else if (bal < -1) && (data < r_node.key) {
                rcase = RotationCase::RL;
            } else {
                rcase = RotationCase::Nil;
            }
        }

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
        l_mut.height = 1 + Self::height(l_mut.left.clone()).max(Self::height(l_mut.right.clone()));

        return l;
    }

    fn leftRotate(node: Tree<T>) -> AVLTree<T> {
        let mut cur = node.borrow_mut();
        let r = cur.right.clone();
        let r_unwrap = r.clone().unwrap();
        let mut r_mut = r_unwrap.borrow_mut();

        let rl = r_mut.left.clone();

        // rotate
        r_mut.left = Some(node.clone());
        cur.right = rl;

        // update height
        cur.height = 1 + Self::height(cur.left.clone()).max(Self::height(cur.right.clone()));
        r_mut.height = 1 + Self::height(r_mut.left.clone()).max(Self::height(r_mut.right.clone()));

        return r;
    }
}


fn main() {
    println!("Hello, world!");
    let root = AVLNode::new(1);
    AVLNode::insert(root.clone(), 2);
    AVLNode::insert(root.clone(), 3);
    AVLNode::insert(root.clone(), 4);
    AVLNode::insert(root.clone(), 5);

    // AVLNode::insert(root, 1);
}
