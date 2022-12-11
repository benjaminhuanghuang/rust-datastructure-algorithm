use std::{
  cell: RefCell, 
  rec:: {Rc, Week}
}


pub struct Node<T:Copy> {
  pub value:T,
  pub next: Option<Rc<RefCell<Node<T>>>,
  pub prev: Option<Week<RefCell<Node<T>>>
}


impl<T:Copy> Node<T> {
  pub fn new(value: T) -> Self {
    Node{
      value,
      next:None
    }
  }
}