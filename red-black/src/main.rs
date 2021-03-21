use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::cell::Ref;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
  Red,
  Black,
}
// type Tree = Rc<RefCell<TreeNode<T>>>;
// type RedBlackTreeNode = Option<Tree>;
#[derive(Debug, Clone)]
struct TreeNode<u32> {
  pub color: NodeColor,
  pub key: u32,
  pub parent: Option<Weak<RefCell<TreeNode<u32>>>>,
  left: Option<Rc<RefCell<TreeNode<u32>>>>,
  right: Option<Rc<RefCell<TreeNode<u32>>>>,
}
#[derive(Debug)]
struct RBTree<u32> {
  root: Option<Rc<RefCell<TreeNode<u32>>>>,
  height: u64
}

impl RBTree<u32> {
  fn new() -> Self {
    Self {root: None, height: 0}
  }


  fn right_rotation1(&mut self, node: &mut Option<Rc<RefCell<TreeNode<u32>>>>) {
    if let None = node {
      println!("Rotation node in 'right_rotation' is None...");
      return;
    }
    let u_node = match node {
      None => return,
      Some(ref n) => n
    };

    let node_left = match u_node.borrow().left {
      None => panic!("Right rotation: this should never be None"),
      Some(ref nl) => Rc::clone(nl)
    };
    let left_right_child = match node_left.borrow().right {
      None => None,
      Some(ref node_left_right) => Some(Rc::clone(node_left_right)),
    };
    u_node.borrow_mut().left = left_right_child;
    
    if !u_node.borrow().left.is_none() {
      match u_node.borrow().left {
        None => panic!("Right rotation: this should never be None"),
        Some(ref nl) => nl.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(u_node)))
      };
    }

    node_left.borrow_mut().parent = match u_node.borrow().parent {
      None => None,
      Some(ref np) => match np.upgrade() {
        None => panic!("Right rotation: this should never be None"),
        Some(ref np) => Some(Rc::downgrade(&Rc::clone(np)))
      }
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
          }
        }
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
      Some(ref n) => n
    };

    let node_right = match u_node.borrow().right {
      None => panic!("Right rotation: this should never be None"),
      Some(ref nl) => Rc::clone(nl)
    };
    let right_left_child = match node_right.borrow().left {
      None => None,
      Some(ref node_right_left) => Some(Rc::clone(node_right_left)),
    };
    u_node.borrow_mut().right = right_left_child;

    if !u_node.borrow().right.is_none() {
      match u_node.borrow().right {
        None => panic!("Right rotation: this should never be None"),
        Some(ref nr) => nr.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(u_node)))
      };
    }

    node_right.borrow_mut().parent = match u_node.borrow().parent {
      None => None,
      Some(ref np) => match np.upgrade() {
        None => panic!("Right rotation: this should never be None"),
        Some(ref np) => Some(Rc::downgrade(&Rc::clone(np)))
      }
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
        }
      }
    }
    node_right.borrow_mut().left = Some(Rc::clone(u_node));
    u_node.borrow_mut().parent = Some(Rc::downgrade(&Rc::clone(&node_right)));
  }

  fn check_property1(&mut self, node: &mut Option<Rc<RefCell<TreeNode<u32>>>>) {
    let mut current_node = node;
    loop {
      let mut node = match current_node {
        None => None,
        Some(ref n) => Some(Rc::clone(n))
      };
     
      let mut parent = match node {
        None => None,
        Some(ref p) => match p.borrow().parent {
          None => None,
          Some(ref p) => match p.upgrade() {
            None => None,
            Some(ref p) => Some(Rc::clone(p))
          }
        }
      };
      if parent.is_none() {
        return;
      }
      let grandparent : Option<Rc<RefCell<TreeNode<u32>>>> = match parent {
        None => None,
        Some(ref gp) => match gp.borrow().parent {
          None => None,
          Some(ref gp) => match gp.upgrade() {
            None => None,
            Some(ref gp) => Some(Rc::clone(gp))
          }
        }
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
          Some(ref gpl) => Some(Rc::clone(gpl))
        }
      };
      let grandparent_right = match grandparent {
        None => None,
        Some(ref gpr) => match gpr.borrow().right {
          None => None,
          Some(ref gpr) => Some(Rc::clone(gpr))
        }
      };

      let mut key_equal = false;
      let p_key = match parent {
        None => return,
        Some(ref p) => {
          match grandparent_left {
            None => {},
            Some(ref gplk) => {
              if p.borrow().key == gplk.borrow().key {
                key_equal = true;
              }
            }
          }
        }
      };

      if key_equal {
        let mut uncle = grandparent_right;
        // Case 1
        match uncle {
          None => {},
          Some(ref u) => {
            if u.borrow().color == NodeColor::Red {
              u.borrow_mut().color = NodeColor::Black;
              match parent {
                None => {panic!("?")},
                Some(ref p) => p.borrow_mut().color = NodeColor::Black
              }
              match grandparent {
                None => {panic!("?")},
                Some(ref gp) => {
                  gp.borrow_mut().color = NodeColor::Red;
                  *current_node =  Some(Rc::clone(gp)); 
                  continue;
                }
              }
            }
          }
        }

        // Case 2
        match parent {
          None => {},
          Some(ref p) => {
            let mut equal = false;
            match p.borrow().right {
              None => {},
              Some(ref pr) => {
                match node {
                  None => {},
                  Some(ref n) => {
                    if pr.borrow().key == n.borrow().key {
                      equal = true;
                    }
                  }
                }
              }
            }
            if equal {
              // INSERT LEFT ROTATE PARENT
              self.left_rotation1(&mut Some(Rc::clone(p)));
              let temp = Some(Rc::clone(p));
              parent = match node {
                None => {panic!("?")},
                Some(ref n) => Some(Rc::clone(n))
              };
              *current_node = temp;
            }
          }
        }
        
        // Case 3
        match parent {
          None => {panic!("?")},
          Some(ref p) => p.borrow_mut().color = NodeColor::Black
        }
        match grandparent {
          None => {panic!("?")},
          Some(ref gp) => gp.borrow_mut().color = NodeColor::Red
        }
        // INSERT RIGHT ROTATE GRANDPARENT
        match grandparent {
          None => {},
          Some(ref gp) => self.right_rotation1(&mut Some(Rc::clone(gp)))
        }
      } else {
        let mut uncle = grandparent_left;
        // Case 1
        match uncle {
          None => {},
          Some(ref u) => {
            if u.borrow().color == NodeColor::Red {
              u.borrow_mut().color = NodeColor::Black;
              match parent {
                None => {panic!("?")},
                Some(ref p) => p.borrow_mut().color = NodeColor::Black
              }
              match grandparent {
                None => {panic!("?")},
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
          None => {},
          Some(ref p) => {
            let mut equal = false;
            match p.borrow().left {
              None => {},
              Some(ref pr) => {
                match node {
                  None => {},
                  Some(ref n) => {
                    if pr.borrow().key == n.borrow().key {
                      equal = true;
                    }
                  }
                }
              }
            }
            if equal {
              // INSERT Right ROTATE PARENT
              self.right_rotation1(&mut Some(Rc::clone(p)));
              let temp = Some(Rc::clone(p));
              parent = match node {
                None => {panic!("?")},
                Some(ref n) => Some(Rc::clone(n))
              };
              *current_node = temp;
            }
          }
        }

        // Case 3
        match parent {
          None => {panic!("?")},
          Some(ref p) => p.borrow_mut().color = NodeColor::Black
        }
        match grandparent {
          None => {panic!("?")},
          Some(ref gp) => gp.borrow_mut().color = NodeColor::Red
        }
        // INSERT LEFT ROTATE GRANDPARENT
        match grandparent {
          None => {},
          Some(ref gp) => self.left_rotation1(&mut Some(Rc::clone(gp)))
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
      let mut y : Option<Rc<RefCell<TreeNode<u32>>>> = None;
      let mut x = match self.root {
          None => {None},
          Some(ref r) => Some(Rc::clone(r))
      };

      while !x.is_none() {
        y = match x {
          None => None,
          Some(ref n) => Some(Rc::clone(n))
        };
        let mut z : Option<Rc<RefCell<TreeNode<u32>>>> = None;
        match x {
          None => {},
          Some(ref x1) => {
            if insert_node.key < x1.borrow().key {
              z = match x1.borrow().left {
                None => None, 
                Some(ref x1l) => Some(Rc::clone(x1l))
              }
            } else {
              z = match x1.borrow().right {
                None => None, 
                Some(ref x1l) => Some(Rc::clone(x1l))
              }
            }
          }
        }
        x = z
      }
      insert_node.parent = match y {
        None => None,
        Some(ref y1) => Some(Rc::downgrade(&Rc::clone(y1)))
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
        None => {},
        Some(ref r) => r.borrow_mut().color = NodeColor::Black
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
      right: None
    }
  }
}

fn main() {
  // let mut tree : TreeNode<u32> = TreeNode::new(3);
  // let mut x:u32 = 1;
  // tree.insert1(x);
  let mut tree = RBTree::new();
  tree.insert(10);
  tree.insert(5);
  tree.insert(1);
  tree.insert(2);
  tree.insert(11);
  tree.insert(9);
  tree.insert(12);
  tree.insert(14);
  tree.insert(0);
  tree.insert(3);
  tree.insert(20);


  println!("{:#?}", tree);
}
