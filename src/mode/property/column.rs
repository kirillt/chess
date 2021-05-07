use ncurses::*;

use crate::location::Location;
use crate::mode::property::Property;
use crate::mode::state::Empty;

#[derive(PartialEq)]
pub enum ColumnOddness {
    Odd,
    Even,
}

impl Property<Empty> for ColumnOddness {
    fn parse(guess: char) -> Option<Box<ColumnOddness>> {
        match guess {
            'o' => Some(Box::new(ColumnOddness::Odd)),
            '1' => Some(Box::new(ColumnOddness::Odd)),
            'e' => Some(Box::new(ColumnOddness::Even)),
            '2' => Some(Box::new(ColumnOddness::Even)),

            _ => None,
        }
    }

    fn calculate(_state: &Empty, location: &Location) -> Box<ColumnOddness> {
        match (location.file + 1) % 2 {
            0 => Box::new(ColumnOddness::Even),
            1 => Box::new(ColumnOddness::Odd),
            _ => panic!(),
        }
    }

    fn print_help() {
        addstr("===================================================================\n");
        addstr("                   COLUMN ODDNESS GUESSING MODE\n");
        addstr("-------------------------------------------------------------------\n");
        addstr("Type [e] or [2] if you think column of specified position is [even]\n");
        addstr("and type [o] or [1] if you think the column is [odd].\n");
        addstr("===================================================================\n");
    }
}
