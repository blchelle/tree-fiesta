use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::{Rc, Weak};
use std::env;
use std::io;

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
}

impl<T> RBTree<T>
where
    T: Ord + Copy + Display + Debug,
{
    /**
     * Creates a new empty tree
    **/
    fn new() -> Self {
        Self {
            root: None
        }
    }

    /**
     * Right rotates the on the node specified
    **/
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

    /**
     * Left rotate on node
    **/
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

    /**
     * Checks and fixes property on tree
    **/
    fn check_property(&mut self, node: &mut Child<T>) {
        let current_node = node;
        loop {
            // Define node nad parent node
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

            // Breaking condition. If parent is none, then exit
            if parent.is_none() {
                return;
            }

            // Define grandparent
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

            // Breaking condition if grandparent is none
            if grandparent.is_none() {
                return;
            }

            match parent {
                None => break,
                Some(ref p) => {
                    // Breaking condition if parent is not red
                    if p.borrow().color != NodeColor::Red {
                        break;
                    }
                }
            };


            // Define left uncle and right uncle
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

            // Determine which is the right uncle
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

            if key_equal { // Uncle is right grandparent
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
            } else {  // uncle is left grandparent
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

    /**
     * Insert a key into the tree
    **/
    fn insert(&mut self, key: T) {
        let mut insert_node = TreeNode::new(key);

        // Checks if the key already exists
        match self.find(key) {
            None => {}
            Some(_) => {
                println!("{} already exists in the tree", key);
                return;
            }
        };

        // If tree is empty
        if self.root.is_none() {
            insert_node.color = NodeColor::Black;
            self.root = Some(Rc::new(RefCell::new(insert_node)));
        } else {
            let mut y: Child<T> = None;
            let mut x = match self.root {
                None => None,
                Some(ref r) => Some(Rc::clone(r)),
            };

            // Determine where to insert the node
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
                // Insert and check propery
                match y {
                    None => panic!("?"),
                    Some(ref y1) => {
                        if insert_node.key < y1.borrow().key {
                            let w = Rc::new(RefCell::new(insert_node));
                            y1.borrow_mut().left = Some(Rc::clone(&w));
                            self.check_property(&mut Some(Rc::clone(&w)));
                        } else {
                            let w = Rc::new(RefCell::new(insert_node));
                            y1.borrow_mut().right = Some(Rc::clone(&w));
                            self.check_property(&mut Some(Rc::clone(&w)));
                        }
                    }
                }
            }

            // Make sure root is black
            match self.root {
                None => {}
                Some(ref r) => r.borrow_mut().color = NodeColor::Black,
            }
        }
    }


    /**
     * Fixes tree after deletion
    **/
    fn fix_delete(&mut self, node: &mut Child<T>, parent: &mut Child<T>) {
        let mut other: Child<T>;
        let mut node = match node {
            None => None,
            Some(ref n) => Some(Rc::clone(n)),
        };
        let mut parent = match parent {
            None => None,
            Some(ref p) => Some(Rc::clone(p)),
        };
        loop {
            let root_key = match self.root {
                None => break,
                Some(ref r) => r.borrow().key,
            };
            let node_key = match node {
                None => break,
                Some(ref n) => n.borrow().key,
            };
            match node {
                // exit conditon
                None => break,
                Some(ref n) => {
                    if !(n.borrow().key != root_key && n.borrow().color == NodeColor::Black) {
                        break;
                    }
                }
            };

            let mut is_left = false;
            match parent {
                None => break,
                Some(ref p) => match p.borrow().left {
                    None => break,
                    Some(ref pl) => {
                        if pl.borrow().key == node_key {
                            other = match p.borrow().right {
                                None => None,
                                Some(ref pr) => Some(Rc::clone(pr)),
                            };
                            is_left = true;
                        } else {
                            other = match p.borrow().left {
                                None => None,
                                Some(ref pl) => Some(Rc::clone(pl)),
                            };
                        }
                    }
                },
            }

            // If node is left child of parent
            if is_left {
                let mut flag = false;
                match other {
                    None => break,
                    Some(ref o) => {
                        // If right child of parent is red
                        // Set color to black
                        // Set parent color to red
                        // Left rotate on parent
                        if o.borrow().color == NodeColor::Red {
                            o.borrow_mut().color = NodeColor::Black;
                            match parent {
                                None => break,
                                Some(ref p) => p.borrow_mut().color = NodeColor::Red,
                            };
                            match parent {
                                None => {}
                                Some(ref p) => self.left_rotation(&mut Some(Rc::clone(p))),
                            };
                            flag = true;
                        }
                    }
                };
                if flag {
                    // reassign right child
                    other = match parent {
                        None => break,
                        Some(ref p) => match p.borrow().right {
                            None => break,
                            Some(ref pr) => Some(Rc::clone(pr)),
                        },
                    };
                }


                let mut matches = false;
                match other {
                    None => break,
                    Some(ref o) => {
                        // Right left color
                        let mut other_left_color = match o.borrow().left {
                            None => break,
                            Some(ref ol) => {
                                if ol.borrow().color == NodeColor::Black {
                                    NodeColor::Black
                                } else {
                                    NodeColor::Red
                                }
                            }
                        };
                        // Right right color
                        let mut other_right_color = match o.borrow().right {
                            None => break,
                            Some(ref or) => {
                                if or.borrow().color == NodeColor::Black {
                                    NodeColor::Black
                                } else {
                                    NodeColor::Red
                                }
                            }
                        };

                        // If both children are black
                        // Set parent to red
                        // Set node to parent
                        // Set parent to node parent
                        if other_left_color == NodeColor::Black
                            && other_right_color == NodeColor::Black
                        {
                            o.borrow_mut().color = NodeColor::Red;
                            node = match parent {
                                None => None,
                                Some(ref p) => Some(Rc::clone(p)),
                            };
                            parent = match node {
                                None => None,
                                Some(ref n) => match n.borrow().parent {
                                    None => None,
                                    Some(ref np) => match np.upgrade() {
                                        None => None,
                                        Some(ref npu) => Some(Rc::clone(npu)),
                                    },
                                },
                            }
                        } else {
                            // If right is black
                            // Set left to black 
                            // Set parent to red
                            // right rotate
                            if other_right_color == NodeColor::Black {
                                match o.borrow().left {
                                    None => break,
                                    Some(ref ol) => ol.borrow_mut().color = NodeColor::Black,
                                };
                                o.borrow_mut().color = NodeColor::Red;
                                self.right_rotation(&mut Some(Rc::clone(o)));
                                matches = true;
                            }
                        }
                    }
                }
                // Reassign node
                if matches {
                    other = match parent {
                        None => break,
                        Some(ref p) => match p.borrow().right {
                            None => break,
                            Some(ref pr) => Some(Rc::clone(pr)),
                        },
                    };
                }

                match other {
                    None => break,
                    Some(ref o) => match parent {
                        None => break,
                        Some(ref p) => {
                            if p.borrow().color == NodeColor::Black {
                                o.borrow_mut().color = NodeColor::Black;
                            } else {
                                o.borrow_mut().color = NodeColor::Red;
                            }
                            p.borrow_mut().color = NodeColor::Black;
                            match o.borrow().right {
                                None => break,
                                Some(ref or) => or.borrow_mut().color = NodeColor::Black,
                            }
                            self.left_rotation(&mut Some(Rc::clone(p)));
                            node = match self.root {
                                None => None,
                                Some(ref r) => Some(Rc::clone(r)),
                            };
                            break;
                        }
                    },
                }
            } else {
                let mut flag = false;
                match other {
                    None => break,
                    Some(ref o) => {
                        if o.borrow().color == NodeColor::Red {
                            o.borrow_mut().color = NodeColor::Black;
                            match parent {
                                None => break,
                                Some(ref p) => p.borrow_mut().color = NodeColor::Red,
                            };
                            match parent {
                                None => {}
                                Some(ref p) => self.right_rotation(&mut Some(Rc::clone(p))),
                            };
                            flag = true;
                        }
                    }
                };
                if flag {
                    other = match parent {
                        None => break,
                        Some(ref p) => match p.borrow().left {
                            None => break,
                            Some(ref pl) => Some(Rc::clone(pl)),
                        },
                    };
                }
                let mut matches = false;
                match other {
                    None => break,
                    Some(ref o) => {
                        let mut other_left_color = match o.borrow().left {
                            None => break,
                            Some(ref ol) => {
                                if ol.borrow().color == NodeColor::Black {
                                    NodeColor::Black
                                } else {
                                    NodeColor::Red
                                }
                            }
                        };
                        let mut other_right_color = match o.borrow().right {
                            None => break,
                            Some(ref or) => {
                                if or.borrow().color == NodeColor::Black {
                                    NodeColor::Black
                                } else {
                                    NodeColor::Red
                                }
                            }
                        };

                        if other_left_color == NodeColor::Black
                            && other_right_color == NodeColor::Black
                        {
                            o.borrow_mut().color = NodeColor::Red;
                            node = match parent {
                                None => None,
                                Some(ref p) => Some(Rc::clone(p)),
                            };
                            parent = match node {
                                None => None,
                                Some(ref n) => match n.borrow().parent {
                                    None => None,
                                    Some(ref np) => match np.upgrade() {
                                        None => None,
                                        Some(ref npu) => Some(Rc::clone(npu)),
                                    },
                                },
                            }
                        } else {
                            if other_left_color == NodeColor::Black {
                                match o.borrow().right {
                                    None => break,
                                    Some(ref or) => or.borrow_mut().color = NodeColor::Black,
                                };
                                o.borrow_mut().color = NodeColor::Red;
                                self.left_rotation(&mut Some(Rc::clone(o)));
                                matches = true;
                            }
                        }
                    }
                }
                if matches {
                    other = match parent {
                        None => break,
                        Some(ref p) => match p.borrow().left {
                            None => break,
                            Some(ref pl) => Some(Rc::clone(pl)),
                        },
                    };
                }
                match other {
                    None => break,
                    Some(ref o) => match parent {
                        None => break,
                        Some(ref p) => {
                            if p.borrow().color == NodeColor::Black {
                                o.borrow_mut().color = NodeColor::Black;
                            } else {
                                o.borrow_mut().color = NodeColor::Red;
                            }
                            p.borrow_mut().color = NodeColor::Black;
                            match o.borrow().left {
                                None => break,
                                Some(ref ol) => ol.borrow_mut().color = NodeColor::Black,
                            }
                            self.right_rotation(&mut Some(Rc::clone(p)));
                            node = match self.root {
                                None => None,
                                Some(ref r) => Some(Rc::clone(r)),
                            };
                            break;
                        }
                    },
                }
            }
        }
    }

    /**
     * Deletes a node from the tree
    **/
    fn delete(&mut self, key: T) {
        fn min_node<T>(node: &Child<T>) -> Child<T> {
            let mut temp = match node {
                None => return None,
                Some(ref n) => Some(Rc::clone(n)),
            };
            loop {
                let mut temp_left = match temp {
                    None => None,
                    Some(ref t) => match t.borrow().left {
                        None => None,
                        Some(ref tl) => Some(Rc::clone(tl)),
                    },
                };
                if temp_left.is_none() {
                    break;
                }
                temp = temp_left;
            }
            return temp;
        }

        let node_to_delete = self.find(key);
        let node_left = match node_to_delete {
            None => return,
            Some(ref n) => match n.borrow().left {
                None => None,
                Some(ref nl) => Some(Rc::clone(nl)),
            },
        };
        let node_right = match node_to_delete {
            None => return,
            Some(ref n) => match n.borrow().right {
                None => None,
                Some(ref nr) => Some(Rc::clone(nr)),
            },
        };
        let root_key = match self.root {
            None => return,
            Some(ref rc) => rc.borrow().key,
        };
        if !node_left.is_none() && !node_right.is_none() {
            let replace = min_node(&node_right);
            let node_key = match node_to_delete {
                None => return,
                Some(ref nc) => nc.borrow().key,
            };

            if node_key == root_key {
                self.root = match replace {
                    None => None,
                    Some(ref r) => Some(Rc::clone(r)),
                };
            } else {
                match node_to_delete {
                    None => return,
                    Some(ref n) => match n.borrow().parent {
                        None => return,
                        Some(ref np) => match np.upgrade() {
                            None => return,
                            Some(ref npu) => {
                                let node_parent_left_key = match Rc::clone(npu).borrow().left {
                                    None => return,
                                    Some(ref nplk) => nplk.borrow().key,
                                };
                                if node_parent_left_key == node_key {
                                    npu.borrow_mut().left = match replace {
                                        None => None,
                                        Some(ref r) => Some(Rc::clone(r)),
                                    };
                                } else {
                                    npu.borrow_mut().right = match replace {
                                        None => None,
                                        Some(ref r) => Some(Rc::clone(r)),
                                    };
                                }
                            }
                        },
                    },
                }
            }

            let child = match replace {
                None => return,
                Some(ref r) => match r.borrow().right {
                    None => None,
                    Some(ref rr) => Some(Rc::clone(rr)),
                },
            };
            let parent = match replace {
                None => return,
                Some(ref r) => match r.borrow().parent {
                    None => None,
                    Some(ref rr) => match rr.upgrade() {
                        None => return,
                        Some(ref rru) => Some(Rc::clone(rru)),
                    },
                },
            };

            let color = match replace {
                None => return,
                Some(ref r) => {
                    if r.borrow().color == NodeColor::Black {
                        NodeColor::Black
                    } else {
                        NodeColor::Red
                    }
                }
            };

            match parent {
                None => {}
                Some(ref p) => {
                    if p.borrow().key == key {
                        match replace {
                            None => None,
                            Some(ref r) => Some(Rc::clone(r)),
                        };
                    } else {
                        if !child.is_none() {
                            match child {
                                None => {}
                                Some(ref c) => {
                                    c.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(p)))
                                }
                            }
                        }
                        p.borrow_mut().left = match child {
                            None => None,
                            Some(ref c) => Some(Rc::clone(c)),
                        };
                        match replace {
                            None => {}
                            Some(ref r) => match node_to_delete {
                                None => return,
                                Some(ref n) => match n.borrow().right {
                                    None => r.borrow_mut().right = None,
                                    Some(ref nr) => {
                                        r.borrow_mut().right = Some(Rc::clone(nr));
                                        nr.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(r)))
                                    }
                                },
                            },
                        }
                    }
                }
            }

            match replace {
                None => return,
                Some(ref r) => {
                    match node_to_delete {
                        None => {}
                        Some(ref n) => {
                            r.borrow_mut().parent = match n.borrow().parent {
                                None => None,
                                Some(ref np) => match np.upgrade() {
                                    None => None,
                                    Some(ref npu) => Some(Rc::downgrade(&Rc::clone(npu))),
                                },
                            };
                            if n.borrow().color == NodeColor::Black {
                                r.borrow_mut().color = NodeColor::Black;
                            } else {
                                r.borrow_mut().color = NodeColor::Red;
                            }
                            r.borrow_mut().left = match n.borrow().left {
                                None => None,
                                Some(ref nl) => Some(Rc::clone(nl)),
                            };
                            match n.borrow().left {
                                None => {}
                                Some(ref nl) => {
                                    nl.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(r)))
                                }
                            }
                        }
                    };
                }
            };

            if color == NodeColor::Black {
                match child {
                    None => return,
                    Some(ref c) => match parent {
                        None => return,
                        Some(ref p) => {
                            self.fix_delete(&mut Some(Rc::clone(c)), &mut Some(Rc::clone(p)))
                        }
                    },
                }
            }
            return;
        }

        let child: Child<T>;
        if !node_left.is_none() {
            child = node_left;
        } else {
            child = node_right;
        }
        let parent = match node_to_delete {
            None => None,
            Some(ref n) => match n.borrow().parent {
                None => None,
                Some(ref np) => match np.upgrade() {
                    None => None,
                    Some(ref npu) => Some(Rc::clone(npu)),
                },
            },
        };
        let color = match node_to_delete {
            None => return,
            Some(ref n) => {
                if n.borrow().color == NodeColor::Black {
                    NodeColor::Black
                } else {
                    NodeColor::Red
                }
            }
        };
        if !child.is_none() {
            match child {
                None => {}
                Some(ref c) => match parent {
                    None => c.borrow_mut().parent = None,
                    Some(ref p) => c.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(p))),
                },
            };
        }
        if root_key == key {
            self.root = match child {
                None => None,
                Some(ref c) => Some(Rc::clone(c)),
            };
        } else {
            match parent {
                None => return,
                Some(ref p) => {
                    let mut found_key = false;
                    let pl_key;
                    match p.borrow().left {
                        None => {}
                        Some(ref pl) => {
                            pl_key = pl.borrow().key;
                            found_key = pl_key == key;
                        }
                    };
                    if found_key {
                        p.borrow_mut().left = match child {
                            None => None,
                            Some(ref c) => Some(Rc::clone(c)),
                        };
                    } else {
                        p.borrow_mut().right = match child {
                            None => None,
                            Some(ref c) => Some(Rc::clone(c)),
                        };
                    }
                }
            }
        }
        if color == NodeColor::Black {
            // DELETE FIXUP CHILD PARENT
            match child {
                None => return,
                Some(ref c) => match parent {
                    None => return,
                    Some(ref p) => {
                        self.fix_delete(&mut Some(Rc::clone(c)), &mut Some(Rc::clone(p)))
                    }
                },
            }
        }
        return;
    }


    /**
     * Finds a node in the tree
    **/
    fn find(&mut self, key: T) -> Child<T> {
        fn recurse<T: Ord + Copy>(node: &mut Child<T>, key: T) -> Child<T> {
            if node.is_none() {
                return None;
            }
            match node {
                None => return None,
                Some(ref n) => {
                    let nkey = n.borrow().key;
                    if nkey == key {
                        return Some(Rc::clone(n));
                    } else if nkey > key {
                        match n.borrow().left {
                            None => return None,
                            Some(ref nl) => return recurse(&mut Some(Rc::clone(nl)), key),
                        };
                    } else {
                        match n.borrow().right {
                            None => return None,
                            Some(ref nr) => return recurse(&mut Some(Rc::clone(nr)), key),
                        };
                    }
                }
            };
        }
        match self.root {
            None => return None,
            Some(ref r) => return recurse(&mut Some(Rc::clone(r)), key),
        }
    }

    /**
     * Counts number of leaf nodes
    **/
    fn count_leaves(&self) -> i32 {
        match self.root {
            None => 0,
            Some(ref root) => root.borrow().count_leaves(),
        }
    }

    /**
     * Returns if the tree is empty or not
    **/
    fn is_empty(&self) -> bool {
        match self.root {
            None => true,
            Some(_) => false,
        }
    }

    /**
     * Returns a inorder vector of the nodes
    **/
    fn inorder_traversal(&self) -> Vec<T> {
        match self.root {
            None => vec![],
            Some(ref root) => root.borrow().inorder_traversal(),
        }
    }

    /**
        algorithmic idea drawn from https://www.baeldung.com/java-print-binary-tree-diagram
    */
    fn pretty_print(root: &Child<T>) -> String {
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

                    let color_string = match n.color {
                        NodeColor::Red => "R",
                        NodeColor::Black => "B",
                    };

                    sb.push_str(color_string);
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

                    let color_string = match n.color {
                        NodeColor::Red => "R",
                        NodeColor::Black => "B",
                    };

                    sb.push_str(color_string);
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

}
