use std::fmt::Debug;

/*  Merge sorting  O(N * log(N))

*/
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
  // sort the left half,
  // sort the right half, O( n * ln (n) ) 1024 * 10 < 1024 * 1024
  // bring the sorted halfs together O(n)

  println!("Merge sorting:{:?}", v);
  if v.len() <= 1 {
    return v;
  }

  let mut res = Vec::with_capacity(v.len());
  let b = v.split_off(v.len() / 2);
  let a = merge_sort(v);
  let b = merge_sort(b);

  // bring them together again add whichever is lowest the front of a or the front of b
  //
  let mut a_it = a.into_iter();
  let mut b_it = b.into_iter();
  let mut a_peek = a_it.next();
  let mut b_peek = b_it.next();

  loop {
    match a_peek {
      Some(ref a_val) => match b_peek {
        Some(ref b_val) => {
          if b_val < a_val {
            res.push(b_peek.take().unwrap());
            b_peek = b_it.next();
          } else {
            res.push(a_peek.take().unwrap());
            a_peek = a_it.next();
          }
        }
        None => {
          res.push(a_peek.take().unwrap());
          res.extend(a_it);
          return res;
        }
      },
      None => {
        if let Some(b_val) = b_peek {
          res.push(b_val);
        }
        res.extend(b_it);
        return res;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_merge_sort() {
    let v = vec![4, 6, 1, 8, 11, 13, 3];
    let v = merge_sort(v);
    assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
  }
}
