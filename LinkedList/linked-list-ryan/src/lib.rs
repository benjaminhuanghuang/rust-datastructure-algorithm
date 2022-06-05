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

pub struct LinkedList<T> {
  head: Link<T>,
}

impl<T> LinkedList<T> {
  // fn empty() -> LinkedList<T> {
  //   LinkedList { head: None }
  // }
  // OR
  fn empty() -> Self {
    Self { head: None }
  }

  fn push(&mut self, element: T) {
    //let old_head = std::mem::replace(&mut self.head, None);
    // OR
    let old_head = self.head.take();
    let new_head = Box::new(LinkNode {
      element,
      next: old_head,
    });

    self.head = Some(new_head);

    // match self.head {
    //   None => {
    //     self.head = Some(Box::new(LinkNode {
    //       element,
    //       next: None,
    //     }))
    //   }
    //   Some(n) => {
    //     let new_head = Some(Box::new(LinkNode {
    //       element,
    //       next: Some(n),
    //     }));
    //     self.head = new_head;
    //   }
    // }
  }

  fn pop(&mut self) -> Option<T> {
    let old_head = self.head.take();
    old_head.map(|n| {
      self.head = n.next;
      n.element
    })
  }

  fn peak(&mut self) -> Option<&T> {
    self.head.as_ref().map(|n| &n.element)
  }
}

struct LinkNode<T> {
  element: T,
  next: Link<T>,
}
type Link<T> = Option<Box<LinkNode<T>>>;

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
    l.push(1);
    l.push(2);
  }
}
