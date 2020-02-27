use std::ops::Mul;
use std::fmt;
use std::fmt::{Formatter, Error};
use crate::errors::DistortedIntError;

pub mod errors;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct DistortedInt {
    value: isize,
    modulus: isize,
    distortion: isize
}

impl DistortedInt {
    pub fn new(value: isize, modulus: isize, distortion: isize) -> Self {
        Self {
            value,
            modulus,
            distortion
        }
    }
}

impl Mul for DistortedInt {
    type Output = Result<Self, DistortedIntError>;

    fn mul(self, rhs: Self) -> Result<Self, DistortedIntError> {
        if self.distortion != rhs.distortion || self.modulus != rhs.modulus {
            return Err(DistortedIntError::MismatchError);
        }
        let new_val = (self.distortion*self.value + (1 - self.distortion)*rhs.value).rem_euclid(self.modulus);
        Ok(Self::new(new_val, self.modulus, self.distortion))
    }
}

impl fmt::Display for DistortedInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "<{} mod {} | {}>", self.value, self.modulus, self.distortion)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
