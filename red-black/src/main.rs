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
struct TreeNode<T: Ord> {
  pub color: NodeColor,
  pub key: T,
  pub parent: Option<Weak<RefCell<TreeNode<T>>>>,
  left: Option<Rc<RefCell<TreeNode<T>>>>,
  right: Option<Rc<RefCell<TreeNode<T>>>>,
}
#[derive(Debug)]
struct RBTree<T: Ord> {
  root: Option<Rc<RefCell<TreeNode<T>>>>,
  height: u64
}

impl<T: Ord> RBTree<T> {
  fn new() -> Self {
    Self {root: None, height: 0}
  }


  /**
   * Performs a right rotation on a tree node
   */
  // fn right_rotate(&mut self, node: &mut TreeNode<T>) {
    // Gets a mutable reference to the left child
    // let &mut left_child = &mut node.left;


    // Gets a reference to the left childs right child

    // Sets the right child of the left child to the current node

    // Changes the parent of the left child to be the parent of the current node

    // Sets the left child of the current node to the right child of the left child

    // Changes the parent of current node to be the left child
  // }

  fn insert(&mut self, key: T) {
    fn insert<T: Ord>(child: &mut Option<Rc<RefCell<TreeNode<T>>>>, mut insert_node: TreeNode<T>) {
      // let mut child = child1.as_ref();
      match child.as_ref() {
        None => {},
        Some(ref c) => {
          if c.borrow().key == insert_node.key {
            return;
          }

          if c.borrow().key > insert_node.key {
            let mut c_mut = c.borrow_mut();

            match c_mut.left {
              None => {
                
                insert_node.parent = Some(Rc::downgrade(&Rc::clone(c)));
                let x = Rc::new(RefCell::new(insert_node));
                c_mut.left = Some(Rc::clone(&x));
                drop(c_mut);
                check_property1(&mut Some(Rc::clone(&x)));
              },
              Some(ref cl) => {
                insert(&mut Some(Rc::clone(cl)), insert_node)
              }
            }
          } else {
            let mut c_mut = c.borrow_mut();

            match c_mut.right {
              None => {

                insert_node.parent = Some(Rc::downgrade(&Rc::clone(c)));
                let x = Rc::new(RefCell::new(insert_node));
                c_mut.right = Some(Rc::clone(&x));
                drop(c_mut);
                check_property1(&mut Some(Rc::clone(&x)));
              },
              Some(ref cl) => {
                insert(&mut Some(Rc::clone(cl)), insert_node)
              }
            }
          }
        }
      }
    }

    fn check_property1<T: Ord>(node: &mut Option<Rc<RefCell<TreeNode<T>>>>) {
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
        let grandparent : Option<Rc<RefCell<TreeNode<T>>>> = match parent {
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
            if p.borrow().color == NodeColor::Red {
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
              None => return,
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
                    node = Some(Rc::clone(gp)); 
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
                let temp = Some(Rc::clone(p));
                parent = match node {
                  None => {panic!("?")},
                  Some(ref n) => Some(Rc::clone(n))
                };
                node = temp;
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
                    node = Some(Rc::clone(gp)); 
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
                let temp = Some(Rc::clone(p));
                parent = match node {
                  None => {panic!("?")},
                  Some(ref n) => Some(Rc::clone(n))
                };
                node = temp;
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
        }
      }
    }

    let mut insert_node = TreeNode::new(key);

    if self.root.is_none() {
      self.height += 1;
      insert_node.color = NodeColor::Black;
      self.root = Some(Rc::new(RefCell::new(insert_node)));
    } else {
      insert(&mut self.root, insert_node);
      match self.root {
        None => {},
        Some(ref r) => r.borrow_mut().color = NodeColor::Black
      }
    }
  }
}

impl<T: Ord> TreeNode<T> {
  fn new(key: T) -> TreeNode<T> {
    return TreeNode {
      color: NodeColor::Red,
      key: key,
      parent: None,
      left: None,
      right: None
    }
  }

  // fn is_none(node : RedBlackTree) -> bool {
  //   match node {
  //     Some(_) => return true,
  //     None => return false
  //   }
  // }
  
  // fn insert1(&mut self, key: T) {
  //   let mut node : Option<Rc<RefCell<TreeNode<T>>>>;
  //   if self.key < key {
  //     match &self.right {
  //       Some(n) => {
  //         n.borrow_mut().insert1(key);
  //       },
  //       None => {self.right = Some(Rc::new(RefCell::new(TreeNode::new(key))))}
  //     }
  //   } else {
  //     match &self.left {
  //       Some(n) => {
  //         n.borrow_mut().insert1(key);
  //       },
  //       None => {self.left = Some(Rc::new(RefCell::new(TreeNode::new(key))))}
  //     }
  //   }

  // }
  // fn insert(self, key: T) {
  //   let mut insert_node = TreeNode::new(key);
  //   let mut parent : Option<Rc<RefCell<TreeNode<T>>>> = None;
  //   let mut node : Option<Rc<RefCell<TreeNode<T>>>> = Some(Rc::new(RefCell::new(self)));

  //   loop {
      
  //     match node {
  //       Some(n) => {
  //         parent = Some(Rc::new(RefCell::new(n)));
  //         if insert_node.key < n.borrow_mut().key {
  //           node = n.borrow_mut().left;
  //         } else {
  //           node = n.borrow_mut().right;
  //         }
  //       },
  //       None => {break}
  //     }
      
  //   }
  //   // insert_node.parent = parent;

  //   // match parent {
  //   //   Some(p) => {
  //   //     if key < p.borrow_mut().key {
  //   //       p.borrow_mut().left = Some(Rc::new(RefCell::new(insert_node)));
  //   //     } else {
  //   //       p.borrow_mut().right = Some(Rc::new(RefCell::new(insert_node)));
  //   //     }
  //   //   },
  //   //   None => {}
  //   // }
  //   // if is_none(parent) {
  //   //   self = node;
  //   // } else {
  //   //   if key < parent.key {
  //   //     parent.left = Some(Rc::new(RefCell::new(insert_node));
  //   //   } else {
  //   //     parent.right = Some(Rc::new(RefCell::new(insert_node));
  //   //   }
  //   // }
  // }
}

fn main() {
  // let mut tree : TreeNode<u32> = TreeNode::new(3);
  // let mut x:u32 = 1;
  // tree.insert1(x);
  let mut tree = RBTree::new();
  tree.insert(3);
  println!("{:#?}", tree);
  tree.insert(4);
  println!("{:#?}", tree);
  tree.insert(1);
  println!("{:#?}", tree);
}
