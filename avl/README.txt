-----
USER MANUAL FOR AVL TREE
-----

//Creating a Tree\\
- Create a tree by calling AVL::new(), a tree will be created with an empty root

//Inserting Elements\\
- Insert elements into the tree by calling tree.insert(val). 

EX:
let tree = AVL::new();
tree.insert(1);

This will insert a new element into the tree.

//Deleting Elements\\
- Deleting eleemtns can be done by calling tree.delete(val).

This will delete the element if it exists in the tree. If the element does not exist in the tree
then the delete function will conduct no action.

//Count number of leaves\\
- tree.count() will return i32 of nodes in the tree

//Height\\
- tree.height() returns the height of the root node in the tree

//Print inorder\\
- tree.inorder() will print the inorder traversal of the tree

//Check empty\\
- tree.isEmpty() will check if the tree is empty

//Print Tree\\
- tree.print() will print the tree in a horizontal manner. Lower is the left child, Higher is right child