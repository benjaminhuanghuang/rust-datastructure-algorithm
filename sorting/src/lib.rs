use std::fmt::Debug;


/*  Bubble sorting  O(N^2)
    pivoit from 0 to len -1
    i from 0 to p - 1
*/
pub fn bubble_sort<T:PartialOrd + Debug> (v: &mut [T]){
    for p in 0..v.len(){
        let mut sorted = true;
        for i in 0..v.len() -1 - p{
            if v[i]> v[i+1] {
                v.swap(i, i+1);
                sorted = false;
            }
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }
}


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


/*  Quick sorting  O(N^2)

*/


//Move first element to the correct place
//Everything lower should be before it, everything highter should be after it
//return it's location
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    // --- Rand p
    // let mut p = b_rand::rand(v.len());
    // v.swap(0, p);
    // p = 0;
    // --- simple version:
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            //move our pivot forward 1, and put this element before it
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1
        }
    }
    p
}


pub fn quick_sort<T: Send + PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("Quick sort {:?}", v);

    let (a, b) = v.split_at_mut(p);
    
    quick_sort(a);
    quick_sort(&mut b[1..]); // middle element already sorted
}


struct RawSend<T>(*mut [T]); //one element tuple

unsafe impl<T> Send for RawSend<T> {}

pub fn threaded_quick_sort<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);

    unsafe {
        let handle = std::thread::spawn(move || {
            threaded_quick_sort(&mut *raw_s.0);
        });

        threaded_quick_sort(&mut b[1..]);

        //compiler doesn't know that we join these
        //We do
        handle.join().ok();
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut v = vec![5,7,0,9];
        bubble_sort(&mut v);
        assert_eq!(v, vec![0,5,7,9]);
    }


    #[test]
    fn test_merge_sort() {
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }


    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
        //assert_eq!(v, vec![1, 3, 4, 6, 19, 8, 11, 13]);
    }


    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 6, 7, 9, 12, 13, 14]);
    }
}
