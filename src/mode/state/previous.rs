use mode::state::ModeState;
use mode::property::KnightDistance;

use location::Location;

use std::fmt::{Display, Formatter, Result};
use std::env::Args;
use std::u8;

pub struct PreviousLocation {
    pub location: Location,
    min_dist: u8,
    max_dist: u8
}

impl PreviousLocation {
    pub fn new(min_dist: u8, max_dist: u8) -> Self {
        PreviousLocation {
            location: Location::random(),
            min_dist, max_dist
        }
    }

    pub fn parse(mut input: Args) -> Self {
        if let Some(arg1) = input.next() {
            match arg1.parse::<u8>() {
                Ok(max_dist) => {
                    let min_dist = input.next()
                        .and_then(|arg2| arg2.parse::<u8>().ok())
                        .unwrap_or(1);
                    PreviousLocation::new(min_dist, max_dist)
                },
                Err(_) => {
                    PreviousLocation::new(1, u8::MAX)
                }
            }
        } else {
            PreviousLocation::new(1, u8::MAX)
        }
    }

    fn in_bounds(&self, location: &Location) -> bool {
        if &self.location != location {
            let distance = KnightDistance::calculate(&self.location, location);
            self.min_dist <= distance && distance <= self.max_dist
        } else {
            false
        }
    }
}

impl Display for PreviousLocation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} -> ", self.location)
    }
}

impl ModeState for PreviousLocation {
    fn next(&self) -> Location {
        let mut location = Location::random();
        while !self.in_bounds(&location) {
            location = Location::random();
        }
        location
    }

    fn tick(&mut self, location: &Location) {
        self.location = location.clone();
    }
}
