#![allow(dead_code)]
#![allow(unused_variables)]

enum Node {
  Empty,
  NonEmpty(u32, Box<Node>),
}

enum Node2<'a> {
  Empty,
  NonEmpty(u32, &'a Node2<'a>),
}

enum Node3 {
  Empty,
  NonEmpty(u32, &'static Node3),
}

/*
 Implement 3:

*/
struct ListNode {
  element: u32,
  next: List,
}

enum List {
  Empty,
  Link(Box<ListNode>),
}

/*
 Implement: 4

*/

pub struct LinkedList {
  head: Link,
}

impl LinkedList {
  fn empty() -> LinkedList {
    LinkedList { head: None }
  }

  fn push(&mut self, element: u32) {
    match self.head {
      None => {
        self.head = Some(Box::new(LinkNode {
          element,
          next: None,
        }))
      }
      Some(_) => {}
    }
  }
}
struct LinkNode {
  element: u32,
  next: Link,
}
type Link = Option<Box<LinkNode>>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn node_works() {
    let list = Node::NonEmpty(32, Box::new(Node::Empty));
  }
  #[test]
  fn node2_works() {
    let node2 = &Node2::Empty;
    let list = Node2::NonEmpty(32, node2);
  }
  #[test]
  fn linked_list_works() {
    let list = List::Link(Box::new(ListNode {
      element: 1,
      next: List::Empty,
    }));
  }
  #[test]
  fn link_works() {
    let link = LinkedList {
      head: Some(Box::new(LinkNode {
        element: 1,
        next: Link::None,
      })),
    };

    let mut l = LinkedList::empty();
    l.push(4);
  }
}
