struct Node<T> {
  data: T,
  next: Option<Rc<Node<T>>>,
  prev: Option<Rc<Node<T>>>,
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
