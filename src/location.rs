use rand::random;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq)]
pub struct Location {
    pub column: u8,
    pub row: u8
}

impl Location {
    pub fn random() -> Self {
        Location {
            column: random::<u8>() % 8,
            row: random::<u8>() % 8
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}",
           ('a' as u8 + self.column) as char,
           self.row + 1)
    }
}