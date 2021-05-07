use mode::property::Property;
use mode::state::Empty;

use color::Color;
use location::Location;

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
        printw("==========================================================\n");
        printw("                    COLOR GUESSING MODE\n");
        printw("----------------------------------------------------------\n");
        printw("Type [w] or [4] if you think specified position is [white]\n");
        printw("and type [b] or [6] if you think the position is [black].\n");
        printw("==========================================================\n");
    }
}
