use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

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

impl<T> RBTree<T>
where
    T: Ord + Copy + Display + Debug,
{
    fn new() -> Self {
        Self {
            root: None,
            height: 0,
        }
    }

    fn right_rotation(&mut self, node: &mut Child<T>) {
        let node = match node {
            None => panic!("Rotation node in 'right_rotation' is None..."),
            Some(ref node) => node,
        };

        // Step 1. Gets a reference to the left child of node
        // Note that if we're performing a right-rotation, this shouldn't be None
        let left_child = match node.borrow().left {
            None => panic!("Right rotation: this should never be None"),
            Some(ref node_left) => Rc::clone(node_left),
        };

        // If the rotation node is the root then we need to move the pointer
        // to the left child
        let root_value = match self.root {
            None => panic!("This should never be none"),
            Some(ref root) => root.borrow().key,
        };

        if root_value == node.borrow().key {
            self.root = Some(Rc::clone(&left_child));
        }

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

        // Step 8. Re-point the parent nodes child pointer that is pointing
        // to the rotation node to the left child
        match parent {
            None => {}
            Some(ref node_parent) => {
                // Gets the value of the parents children
                let is_left_child = match node_parent.borrow().left {
                    None => false,
                    Some(ref node_parent_left) => {
                        node.borrow().key == node_parent_left.borrow().key
                    }
                };

                if is_left_child {
                    node_parent.borrow_mut().left = Some(Rc::clone(&left_child));
                    return;
                }

                let is_right_child = match node_parent.borrow().right {
                    None => false,
                    Some(ref node_parent_right) => {
                        node.borrow().key == node_parent_right.borrow().key
                    }
                };

                if is_right_child {
                    node_parent.borrow_mut().right = Some(Rc::clone(&left_child));
                    return;
                }
            }
        }
    }

    fn left_rotation(&mut self, node: &mut Child<T>) {
        let node = match node {
            None => panic!("Rotation node in 'right_rotation' is None..."),
            Some(ref node) => node,
        };

        // Step 1. Gets a reference to the right child of node
        // Note that if we're performing a left-rotation, this shouldn't be None
        let right_child = match node.borrow().right {
            None => panic!("Left rotation: this should never be None"),
            Some(ref node_right) => Rc::clone(node_right),
        };

        // If the rotation node is the root then we need to move the pointer
        // to the left child
        let root_value = match self.root {
            None => panic!("This should never be none"),
            Some(ref root) => root.borrow().key,
        };

        if root_value == node.borrow().key {
            self.root = Some(Rc::clone(&right_child));
        }

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

        // Step 8. Re-point the parent nodes child pointer that is pointing
        // to the rotation node to the left child
        match parent {
            None => {}
            Some(ref node_parent) => {
                // Gets the value of the parents children
                let is_left_child = match node_parent.borrow().left {
                    None => false,
                    Some(ref node_parent_left) => {
                        node.borrow().key == node_parent_left.borrow().key
                    }
                };

                if is_left_child {
                    node_parent.borrow_mut().left = Some(Rc::clone(&right_child));
                    return;
                }

                let is_right_child = match node_parent.borrow().right {
                    None => false,
                    Some(ref node_parent_right) => {
                        node.borrow().key == node_parent_right.borrow().key
                    }
                };

                if is_right_child {
                    node_parent.borrow_mut().right = Some(Rc::clone(&right_child));
                    return;
                }
            }
        }
    }

    fn check_property1(&mut self, node: &mut Child<T>) {
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
            let grandparent: Child<T> = match parent {
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
                            self.left_rotation(&mut Some(Rc::clone(p)));
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
                    Some(ref gp) => self.right_rotation(&mut Some(Rc::clone(gp))),
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
                            self.right_rotation(&mut Some(Rc::clone(p)));
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
                    Some(ref gp) => self.left_rotation(&mut Some(Rc::clone(gp))),
                }
            }
        }
    }

    fn insert(&mut self, key: T) {
        let mut insert_node = TreeNode::new(key);

        if self.root.is_none() {
            self.height += 1;
            insert_node.color = NodeColor::Black;
            self.root = Some(Rc::new(RefCell::new(insert_node)));
        } else {
            let mut y: Child<T> = None;
            let mut x = match self.root {
                None => None,
                Some(ref r) => Some(Rc::clone(r)),
            };

            while !x.is_none() {
                y = match x {
                    None => None,
                    Some(ref n) => Some(Rc::clone(n)),
                };
                let mut z: Child<T> = None;
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

    fn inorder_traversal(&self) -> Vec<T> {
        match self.root {
            None => vec![],
            Some(ref root) => root.borrow().inorder_traversal(),
        }
    }

    /**
        algorithmic idea drawn from https://www.baeldung.com/java-print-binary-tree-diagram
    */
    fn pretty_print(root: Child<T>) -> String {
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
        node: Child<T>,
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
    tree.insert(9);
    tree.insert(8);
    tree.insert(7);
    tree.insert(6);
    tree.insert(5);
    tree.insert(4);
    tree.insert(3);
    tree.insert(11);
    tree.insert(12);
    tree.insert(13);
    tree.insert(14);
    tree.insert(15);

    println!("{}", tree.count_leaves());
    println!("The tree is empty: {}", tree.is_empty());
    println!("{:?}", tree.inorder_traversal());
    println!("{}", RBTree::pretty_print(tree.root));
}
