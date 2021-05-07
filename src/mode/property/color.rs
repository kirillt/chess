use crate::mode::property::Property;
use crate::mode::state::Empty;

use crate::color::Color;
use crate::location::Location;

use ncurses::*;

impl Property<Empty> for Color {
    fn parse(guess: char) -> Option<Box<Color>> {
        match guess {
            'w' => Some(Box::new(Color::White)),
            '4' => Some(Box::new(Color::White)),
            'b' => Some(Box::new(Color::Black)),
            '6' => Some(Box::new(Color::Black)),

            _ => None,
        }
    }

    fn calculate(_state: &Empty, location: &Location) -> Box<Color> {
        Box::new(if location.rank % 2 == location.file % 2 {
            Color::Black
        } else {
            Color::White
        })
    }

    fn print_help() {
        addstr("==========================================================\n");
        addstr("                    COLOR GUESSING MODE\n");
        addstr("----------------------------------------------------------\n");
        addstr("Type [w] or [4] if you think specified position is [white]\n");
        addstr("and type [b] or [6] if you think the position is [black].\n");
        addstr("==========================================================\n");
    }
}
