
pub struct BinarySearchTreeNode<T:PartialOrd> {
  value: T,
  left: Option<Box<BinarySearchTreeNode<T>>>,
  right: Option<Box<BinarySearchTreeNode<T>>>,
}

impl<T> BinarySearchTreeNode<T>
where
  T: PartialEq + PartialOrd,
{
  #[inline]
  pub fn new(x: T) -> BinarySearchTreeNode<T> {
    BinarySearchTreeNode {
      value: x,
      left: None,
      right: None,
    }
  }
}

pub struct BinarySearchTree<T:PartialOrd> {
  pub root: BinarySearchTreeNode<T>
}