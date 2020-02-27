use distorted_ints::DistortedInt;
use std::cmp::Ordering;

pub fn has_distorted_idempotent_property(n: isize, alpha: isize) -> bool {
    for val in 1..n {
        let x = DistortedInt::new(val, n, alpha);
        match (x*x).unwrap().cmp(&x) {
            Ordering::Equal => {},
            _ => return false
        }
    }
    return true;
}