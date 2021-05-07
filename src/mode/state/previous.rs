use std::env::Args;
use std::fmt::{Display, Formatter, Result};
use std::u8;

use crate::location::Location;
use crate::mode::property::KnightDistance;
use crate::mode::state::ModeState;

pub struct PreviousLocation {
    pub location: Location,
    min_dist: u8,
    max_dist: u8,
}

impl PreviousLocation {
    pub fn new(min_dist: u8, max_dist: u8) -> Self {
        PreviousLocation {
            location: Location::random(),
            min_dist,
            max_dist,
        }
    }

    pub fn parse(mut input: Args) -> Self {
        input
            .next()
            .and_then(|s| s.parse::<u8>().ok())
            .filter(|min_dist| *min_dist > 0)
            .and_then(|min_dist| {
                input
                    .next()
                    .and_then(|arg| arg.parse::<u8>().ok())
                    .filter(|max_dist| *max_dist < 6 && min_dist <= *max_dist)
                    .map(|max_dist| PreviousLocation::new(min_dist, max_dist))
            })
            .unwrap_or_else(|| {
                println!("Wrong arguments to Knight Mode, using default configuration.");
                PreviousLocation::new(1, u8::MAX)
            })
    }

    fn in_bounds(&self, location: &Location) -> bool {
        if &self.location != location {
            let distance = KnightDistance::distance(&self.location, location);
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
