use mode::state::ModeState;
use color::Color;

use std::fmt::{Display, Formatter, Result};
use std::env::Args;

pub struct SideContainer {
    pub side: Color,
    period: Option<u8>,
    countdown: u8
}

impl SideContainer {
    pub fn new(side: Color, period: Option<u8>) -> Self {
        SideContainer {
            side: side,
            period: period,
            countdown: period.unwrap_or(0)
        }
    }
}

impl SideContainer {
    pub fn parse(mut input: Args) -> Self {
        if let Some(arg1) = input.next() {
            match Color::parse_str(&arg1[..]) {
                Some(side) => {
                    let period = input.next()
                        .and_then(|arg2| arg2.parse::<u8>().ok());
                    SideContainer::new(side, period)
                },
                None => {
                    let period = arg1.parse::<u8>().ok();
                    SideContainer::new(Color::Black, period)
                }
            }
        } else {
            SideContainer::new(Color::White, Some(4))
        }
    }
}

impl Display for SideContainer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, " <{}> ", self.side.print())
    }
}

impl ModeState for SideContainer {
    fn tick(&mut self) {
        match self.period {
            Some(period) => {
                self.countdown -= 1;
                if self.countdown == 0 {
                    self.side = self.side.invert();
                    self.countdown = period;
                }
            },
            None => {}
        }
    }
}