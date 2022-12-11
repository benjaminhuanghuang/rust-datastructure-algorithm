#![allow(dead_code)]
#![allow(unused_variables)]

struct Node<T> {
  element: T,
  next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
  head: Link<T>,
}

impl<T> LinkedList<T> {
  // constructor
  // fn empty() -> LinkedList<T> {
  //   LinkedList { head: None }
  // }
  // OR
  fn empty() -> Self {
    Self { head: None }
  }

  //use the new node as the head of the list
  fn push(&mut self, element: T) {
    //let old_head = std::mem::replace(&mut self.head, None);
    // at this moment, self.head is None
    // OR
    let old_head: Option<Box<Node<T>>> = self.head.take();
    let new_head = Box::new(Node {
      element,
      next: old_head,
    });

    self.head = Some(new_head);
  }

  /*
    Error! self.head move out the data
  */
  //use the new node as the head of the list
  // fn push_naive(&mut self, element: T) {
  //   match self.head {
  //     // mat
  //     None => {
  //       self.head = Some(Box::new(Node {
  //         element,
  //         next: None,
  //       }))
  //     }
  //     Some(n) => {
  //       let new_head = Some(Box::new(Node {
  //         element,
  //         next: Some(n),
  //       }));
  //       self.head = new_head;
  //     }
  //   }
  // }

  fn pop(&mut self) -> Option<T> {
    let old_head = self.head.take();
    match old_head {
      Some(n) => {
        self.head = n.next;
        Some(n.element)
      }
      None => None,
    }
  }

  fn pop_2(&mut self) -> Option<T> {
    let old_head = self.head.take();
    old_head.map(|n| {
      self.head = n.next;
      n.element
    })
  }

  fn peek(&self) -> Option<&T> {
    match &self.head {
      Some(n) => Some(&n.element),
      None => None,
    }
  }

  fn peek2(&self) -> Option<&T> {
    // as_ref() borrow self.head
    self.head.as_ref().map(|n| &n.element)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn node_works() {
    let mut list = LinkedList::empty();
    list.push(1);
    list.push(2);
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
