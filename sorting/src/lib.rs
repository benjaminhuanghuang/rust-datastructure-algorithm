use std::fmt::Debug;

// O(N^2)
pub fn buble_sort<T:PartialOrd + Debug> (v: &mut [T]){
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut v = vec![5,7,0,9];
        buble_sort(&mut v);
        assert_eq!(v, vec![0,5,7,9]);
    }
}
