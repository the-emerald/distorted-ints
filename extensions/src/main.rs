use distorted_ints::DistortedInt;
use distorted_ints::errors::DistortedIntError;
use crate::easy::has_distorted_idempotent_property;

mod easy;

const DIVIDER: &str = "----------";

fn main() {
    let x = DistortedInt::new(2, 5, 3);
    println!("x = {}", x);
    let y = DistortedInt::new(4, 5, 3);
    println!("y = {}", y);

    match x*x {
        Ok(t) => println!("x*x = {}", t),
        Err(e) => panic!("{}", e)
    }

    match x*y {
        Ok(t) => println!("x*y = {}", t),
        Err(e) => panic!("{}", e)
    }

    println!("{}", DIVIDER);
    println!("has_distorted_idempotent_property:");
    for n in 1..101 {
        for a in 0..n {
            match has_distorted_idempotent_property(n, a) {
                true => {},
                false => {println!("Property does not hold for {} {}.", n, a)} //
            }
        }
    }



}