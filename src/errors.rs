#[derive(Debug)]
pub enum DistortedIntError {
    MismatchError
}

impl std::error::Error for DistortedIntError {}

impl std::fmt::Display for DistortedIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
           -> std::fmt::Result {
        match self {
            DistortedIntError::MismatchError => write!(f, ""),
        }
    }
}