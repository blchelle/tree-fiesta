use std::cell::Ref;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}
// type Tree = Rc<RefCell<TreeNode<T>>>;
// type RedBlackTreeNode = Option<Tree>;

type Child<T> = Option<Rc<RefCell<TreeNode<T>>>>;
type Parent<T> = Option<Weak<RefCell<TreeNode<T>>>>;

#[derive(Debug, Clone)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: Parent<T>,
    left: Child<T>,
    right: Child<T>,
}
#[derive(Debug)]
struct RBTree<T> {
    root: Child<T>,
    height: u64,
}

impl RBTree<u32> {
    fn new() -> Self {
        Self {
            root: None,
            height: 0,
        }
    }

    fn right_rotation1(&mut self, node: &mut Option<Rc<RefCell<TreeNode<u32>>>>) {
        if let None = node {
            println!("Rotation node in 'right_rotation' is None...");
            return;
        }
        let u_node = match node {
            None => return,
            Some(ref n) => n,
        };

        let node_left = match u_node.borrow().left {
            None => panic!("Right rotation: this should never be None"),
            Some(ref nl) => Rc::clone(nl),
        };
        let left_right_child = match node_left.borrow().right {
            None => None,
            Some(ref node_left_right) => Some(Rc::clone(node_left_right)),
        };
        u_node.borrow_mut().left = left_right_child;
        if !u_node.borrow().left.is_none() {
            match u_node.borrow().left {
                None => panic!("Right rotation: this should never be None"),
                Some(ref nl) => nl.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(u_node))),
            };
        }

        node_left.borrow_mut().parent = match u_node.borrow().parent {
            None => None,
            Some(ref np) => match np.upgrade() {
                None => panic!("Right rotation: this should never be None"),
                Some(ref np) => Some(Rc::downgrade(&Rc::clone(np))),
            },
        };

        if u_node.borrow().parent.is_none() {
            self.root = Some(Rc::clone(&node_left));
        } else {
            match u_node.borrow().parent {
                None => panic!("Right rotation: this should never be None"),
                Some(ref up) => match up.upgrade() {
                    None => panic!("Right rotation: this should never be None"),
                    Some(ref up) => match up.borrow().left {
                        None => panic!("Right rotation: this should never be None"),
                        Some(ref upl) => {
                            if u_node.borrow().key == upl.borrow().key {
                                up.borrow_mut().left = Some(Rc::clone(&node_left));
                            } else {
                                up.borrow_mut().right = Some(Rc::clone(&node_left));
                            }
                        }
                    },
                },
            }
        }

        node_left.borrow_mut().right = Some(Rc::clone(u_node));
        u_node.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(&node_left)));
    }

    fn left_rotation1(&mut self, node: &mut Option<Rc<RefCell<TreeNode<u32>>>>) {
        if let None = node {
            println!("Rotation node in 'right_rotation' is None...");
            return;
        }
        let u_node = match node {
            None => return,
            Some(ref n) => n,
        };

        let node_right = match u_node.borrow().right {
            None => panic!("Right rotation: this should never be None"),
            Some(ref nl) => Rc::clone(nl),
        };
        let right_left_child = match node_right.borrow().left {
            None => None,
            Some(ref node_right_left) => Some(Rc::clone(node_right_left)),
        };
        u_node.borrow_mut().right = right_left_child;

        if !u_node.borrow().right.is_none() {
            match u_node.borrow().right {
                None => panic!("Right rotation: this should never be None"),
                Some(ref nr) => nr.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(u_node))),
            };
        }

        node_right.borrow_mut().parent = match u_node.borrow().parent {
            None => None,
            Some(ref np) => match np.upgrade() {
                None => panic!("Right rotation: this should never be None"),
                Some(ref np) => Some(Rc::downgrade(&Rc::clone(np))),
            },
        };

        if u_node.borrow().parent.is_none() {
            self.root = Some(Rc::clone(&node_right));
        } else {
            let mut is_equal = false;
            match u_node.borrow().parent {
                None => panic!("Right rotation: this should never be None"),
                Some(ref up) => match up.upgrade() {
                    None => panic!("Right rotation: this should never be None"),
                    Some(ref up) => {
                        match up.borrow().left {
                            None => panic!("Right rotation: this should never be None"),
                            Some(ref upl) => {
                                if u_node.borrow().key == upl.borrow().key {
                                    is_equal = true;
                                }
                            }
                        };
                        if is_equal {
                            up.borrow_mut().left = Some(Rc::clone(&node_right));
                        } else {
                            up.borrow_mut().right = Some(Rc::clone(&node_right));
                        }
                    }
                },
            }
        }
        node_right.borrow_mut().left = Some(Rc::clone(u_node));
        u_node.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(&node_right)));
    }

    fn check_property1(&mut self, node: &mut Option<Rc<RefCell<TreeNode<u32>>>>) {
        let current_node = node;
        loop {
            let node = match current_node {
                None => None,
                Some(ref n) => Some(Rc::clone(n)),
            };
            let mut parent = match node {
                None => None,
                Some(ref p) => match p.borrow().parent {
                    None => None,
                    Some(ref p) => match p.upgrade() {
                        None => None,
                        Some(ref p) => Some(Rc::clone(p)),
                    },
                },
            };
            if parent.is_none() {
                return;
            }
            let grandparent: Option<Rc<RefCell<TreeNode<u32>>>> = match parent {
                None => None,
                Some(ref gp) => match gp.borrow().parent {
                    None => None,
                    Some(ref gp) => match gp.upgrade() {
                        None => None,
                        Some(ref gp) => Some(Rc::clone(gp)),
                    },
                },
            };
            if grandparent.is_none() {
                return;
            }

            match parent {
                None => break,
                Some(ref p) => {
                    if p.borrow().color != NodeColor::Red {
                        break;
                    }
                }
            };

            let grandparent_left = match grandparent {
                None => None,
                Some(ref gpl) => match gpl.borrow().left {
                    None => None,
                    Some(ref gpl) => Some(Rc::clone(gpl)),
                },
            };
            let grandparent_right = match grandparent {
                None => None,
                Some(ref gpr) => match gpr.borrow().right {
                    None => None,
                    Some(ref gpr) => Some(Rc::clone(gpr)),
                },
            };

            let mut key_equal = false;
            match parent {
                None => return,
                Some(ref p) => match grandparent_left {
                    None => {}
                    Some(ref gplk) => {
                        if p.borrow().key == gplk.borrow().key {
                            key_equal = true;
                        }
                    }
                },
            };

            if key_equal {
                let uncle = grandparent_right;
                // Case 1
                match uncle {
                    None => {}
                    Some(ref u) => {
                        if u.borrow().color == NodeColor::Red {
                            u.borrow_mut().color = NodeColor::Black;
                            match parent {
                                None => {
                                    panic!("?")
                                }
                                Some(ref p) => p.borrow_mut().color = NodeColor::Black,
                            }
                            match grandparent {
                                None => {
                                    panic!("?")
                                }
                                Some(ref gp) => {
                                    gp.borrow_mut().color = NodeColor::Red;
                                    *current_node = Some(Rc::clone(gp));
                                    continue;
                                }
                            }
                        }
                    }
                }

                // Case 2
                match parent {
                    None => {}
                    Some(ref p) => {
                        let mut equal = false;
                        match p.borrow().right {
                            None => {}
                            Some(ref pr) => match node {
                                None => {}
                                Some(ref n) => {
                                    if pr.borrow().key == n.borrow().key {
                                        equal = true;
                                    }
                                }
                            },
                        }
                        if equal {
                            // INSERT LEFT ROTATE PARENT
                            self.left_rotation1(&mut Some(Rc::clone(p)));
                            let temp = Some(Rc::clone(p));
                            parent = match node {
                                None => {
                                    panic!("?")
                                }
                                Some(ref n) => Some(Rc::clone(n)),
                            };
                            *current_node = temp;
                        }
                    }
                }
                // Case 3
                match parent {
                    None => {
                        panic!("?")
                    }
                    Some(ref p) => p.borrow_mut().color = NodeColor::Black,
                }
                match grandparent {
                    None => {
                        panic!("?")
                    }
                    Some(ref gp) => gp.borrow_mut().color = NodeColor::Red,
                }
                // INSERT RIGHT ROTATE GRANDPARENT
                match grandparent {
                    None => {}
                    Some(ref gp) => self.right_rotation1(&mut Some(Rc::clone(gp))),
                }
            } else {
                let uncle = grandparent_left;
                // Case 1
                match uncle {
                    None => {}
                    Some(ref u) => {
                        if u.borrow().color == NodeColor::Red {
                            u.borrow_mut().color = NodeColor::Black;
                            match parent {
                                None => {
                                    panic!("?")
                                }
                                Some(ref p) => p.borrow_mut().color = NodeColor::Black,
                            }
                            match grandparent {
                                None => {
                                    panic!("?")
                                }
                                Some(ref gp) => {
                                    gp.borrow_mut().color = NodeColor::Red;
                                    *current_node = Some(Rc::clone(gp));
                                    continue;
                                }
                            }
                        }
                    }
                }

                // Case 2
                match parent {
                    None => {}
                    Some(ref p) => {
                        let mut equal = false;
                        match p.borrow().left {
                            None => {}
                            Some(ref pr) => match node {
                                None => {}
                                Some(ref n) => {
                                    if pr.borrow().key == n.borrow().key {
                                        equal = true;
                                    }
                                }
                            },
                        }
                        if equal {
                            // INSERT Right ROTATE PARENT
                            self.right_rotation1(&mut Some(Rc::clone(p)));
                            let temp = Some(Rc::clone(p));
                            parent = match node {
                                None => {
                                    panic!("?")
                                }
                                Some(ref n) => Some(Rc::clone(n)),
                            };
                            *current_node = temp;
                        }
                    }
                }

                // Case 3
                match parent {
                    None => {
                        panic!("?")
                    }
                    Some(ref p) => p.borrow_mut().color = NodeColor::Black,
                }
                match grandparent {
                    None => {
                        panic!("?")
                    }
                    Some(ref gp) => gp.borrow_mut().color = NodeColor::Red,
                }
                // INSERT LEFT ROTATE GRANDPARENT
                match grandparent {
                    None => {}
                    Some(ref gp) => self.left_rotation1(&mut Some(Rc::clone(gp))),
                }
            }
        }
    }

    fn insert(&mut self, key: u32) {
        let mut insert_node = TreeNode::new(key);

        if self.root.is_none() {
            self.height += 1;
            insert_node.color = NodeColor::Black;
            self.root = Some(Rc::new(RefCell::new(insert_node)));
        } else {
            let mut y: Option<Rc<RefCell<TreeNode<u32>>>> = None;
            let mut x = match self.root {
                None => None,
                Some(ref r) => Some(Rc::clone(r)),
            };

            while !x.is_none() {
                y = match x {
                    None => None,
                    Some(ref n) => Some(Rc::clone(n)),
                };
                let mut z: Option<Rc<RefCell<TreeNode<u32>>>> = None;
                match x {
                    None => {}
                    Some(ref x1) => {
                        if insert_node.key < x1.borrow().key {
                            z = match x1.borrow().left {
                                None => None,
                                Some(ref x1l) => Some(Rc::clone(x1l)),
                            }
                        } else {
                            z = match x1.borrow().right {
                                None => None,
                                Some(ref x1l) => Some(Rc::clone(x1l)),
                            }
                        }
                    }
                }
                x = z
            }
            insert_node.parent = match y {
                None => None,
                Some(ref y1) => Some(Rc::downgrade(&Rc::clone(y1))),
            };
            if y.is_none() {
                self.root = Some(Rc::new(RefCell::new(insert_node)));
            } else {
                match y {
                    None => panic!("?"),
                    Some(ref y1) => {
                        if insert_node.key < y1.borrow().key {
                            let w = Rc::new(RefCell::new(insert_node));
                            y1.borrow_mut().left = Some(Rc::clone(&w));
                            self.check_property1(&mut Some(Rc::clone(&w)));
                        } else {
                            let w = Rc::new(RefCell::new(insert_node));
                            y1.borrow_mut().right = Some(Rc::clone(&w));
                            self.check_property1(&mut Some(Rc::clone(&w)));
                        }
                    }
                }
            }

            // insert(&mut self.root, insert_node);
            match self.root {
                None => {}
                Some(ref r) => r.borrow_mut().color = NodeColor::Black,
            }
        }
    }

    fn count_leaves(&self) -> i32 {
        match self.root {
            None => 0,
            Some(ref root) => root.borrow().count_leaves(),
        }
    }

    fn is_empty(&self) -> bool {
        match self.root {
            None => true,
            Some(_) => false,
        }
    }

    fn inorder_traversal(&self) -> Vec<u32> {
        match self.root {
            None => vec![],
            Some(ref root) => root.borrow().inorder_traversal(),
        }
    }

    /**
        algorithmic idea drawn from https://www.baeldung.com/java-print-binary-tree-diagram
    */
    fn pretty_print(root: Option<Rc<RefCell<TreeNode<u32>>>>) -> String {
        match root {
            None => {
                return "".to_string();
            }
            Some(node) => {
                let rc_sb = Rc::from(RefCell::from(String::from("")));
                let n = node.borrow();

                {
                    let mut sb = rc_sb.borrow_mut();
                    sb.push_str(n.key.clone().to_string().as_str());
                }

                let pointer_right = "└──";
                let pointer_left;

                if n.right.is_none() {
                    pointer_left = "└──";
                } else {
                    pointer_left = "├──";
                }

                Self::pretty_print_helper(
                    rc_sb.clone(),
                    "",
                    pointer_left,
                    n.left.clone(),
                    n.right.is_some(),
                );
                Self::pretty_print_helper(rc_sb.clone(), "", pointer_right, n.right.clone(), false);

                return rc_sb.borrow_mut().clone();
            }
        }
    }

    fn pretty_print_helper(
        rc_sb: Rc<RefCell<String>>,
        padding: &str,
        pointer: &str,
        node: Option<Rc<RefCell<TreeNode<u32>>>>,
        has_right: bool,
    ) {
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
                if has_right {
                    psb.push_str("│  ");
                } else {
                    psb.push_str("   ");
                }

                let both_pad = String::from(psb);
                let pointer_right = "└──";
                let pointer_left;

                if n.right.is_none() {
                    pointer_left = "└──";
                } else {
                    pointer_left = "├──";
                }

                Self::pretty_print_helper(
                    rc_sb.clone(),
                    &*both_pad,
                    pointer_left,
                    n.left.clone(),
                    n.right.is_some(),
                );

                Self::pretty_print_helper(
                    rc_sb.clone(),
                    &*both_pad,
                    pointer_right,
                    n.right.clone(),
                    false,
                );
            }
        }
    }
}

impl<T: Ord + Copy> TreeNode<T> {
    fn new(key: T) -> TreeNode<T> {
        return TreeNode {
            color: NodeColor::Red,
            key: key,
            parent: None,
            left: None,
            right: None,
        };
    }

    fn count_leaves(&self) -> i32 {
        // Node has no children which means that it is a leaf
        if self.left.is_none() && self.right.is_none() {
            return 1;
        }

        let mut leaf_count = 0;

        // If the left child isn't none, traverse into it
        if let Some(ref left) = self.left {
            leaf_count += left.borrow().count_leaves();
        }

        // If the right child isn't none, traverse into it
        if let Some(ref right) = self.right {
            leaf_count += right.borrow().count_leaves();
        }

        leaf_count
    }

    fn inorder_traversal(&self) -> Vec<T> {
        let mut node_values = vec![];

        // If the left child isn't none, traverse into it
        if let Some(ref left) = self.left {
            node_values.append(&mut left.borrow().inorder_traversal());
        }

        // Push the current nodes values
        node_values.push(self.key);

        // If the right child isn't none, traverse into it
        if let Some(ref right) = self.right {
            node_values.append(&mut right.borrow().inorder_traversal());
        }

        node_values
    }
}

fn main() {
    let mut tree = RBTree::new();
    println!("The tree is empty: {}", tree.is_empty());

    tree.insert(10);
    tree.insert(5);
    tree.insert(1);
    tree.insert(7);

    println!("{}", tree.count_leaves());
    println!("The tree is empty: {}", tree.is_empty());
    println!("{:?}", tree.inorder_traversal());
    println!("{}", RBTree::pretty_print(tree.root));
}
