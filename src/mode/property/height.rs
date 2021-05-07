use crate::mode::property::Property;
use crate::mode::state::Empty;

use crate::location::Location;

use ncurses::*;
use std::cmp::min;

#[derive(PartialEq)]
pub struct Height {
    value: u8,
}

impl Property<Empty> for Height {
    fn parse(guess: char) -> Option<Box<Height>> {
        format!("{}", guess).parse::<u8>().ok().and_then(|x| {
            if x < 4 {
                Some(Box::new(Height { value: x }))
            } else {
                None
            }
        })
    }

    fn calculate(_state: &Empty, location: &Location) -> Box<Height> {
        fn mirror(x: u8) -> u8 {
            let x = x as i8 - 4;
            let x = if x < 0 { x.abs() - 1 } else { x };
            x as u8
        };

        Box::new(Height {
            value: min(3 - mirror(location.file), 3 - mirror(location.rank)),
        })
    }

    fn print_help() {
        printw("==========================================================\n");
        printw("                    HEIGHT GUESSING MODE\n");
        printw("----------------------------------------------------------\n");
        printw("Type [0], [1], [2] or [3] if you think specified position\n");
        printw("belongs to outermost circle, outer or inner one or center.\n");
        printw("==========================================================\n");
    }
}
