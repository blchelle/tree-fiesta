use std::cell::RefCell;
use std::rc::Rc;

type Tree = Rc<RefCell<AVLNode<u32>>>;
type AVLTree= Option<Tree>;
struct AVLNode<T> {
    pub key: T,
    height: i32,
    left: AVLTree,
    right: AVLTree,
}



// impl<T> AVLNode {
//
// }

