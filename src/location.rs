use rand::random;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq)]
pub struct Location {
    pub file: u8,
    pub rank: u8,
}

impl Location {
    pub fn random() -> Self {
        Location {
            file: random::<u8>() % 8,
            rank: random::<u8>() % 8,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}", ('a' as u8 + self.file) as char, self.rank + 1)
    }
}
