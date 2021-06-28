

use binary_tree::{BinaryTree, TreeNode};


fn main() {
  let mut bt_i32:BinaryTree<i32> = BinaryTree::new();

  let mut root = TreeNode::<i32>::new(0);
  let left = TreeNode::<i32>::new(1);
  let right = TreeNode::<i32>::new(2);

  root.left = Some(Box::new(left));
  root.right = Some(Box::new(right));
  bt_i32.root = Some(Box::new(root));
}