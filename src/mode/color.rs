use mode::*;

use ncurses::*;

#[derive(PartialEq)]
pub enum Color { White, Black }

impl Property for Color {
    fn parse(guess: char) -> Option<Box<Color>> {
        match guess {
            'w' => Some(box Color::White),
            '4' => Some(box Color::White),
            'b' => Some(box Color::Black),
            '6' => Some(box Color::Black),

            _ => None
        }
    }

    fn calculate(column: u8, row: u8) -> Box<Color> {
        box if row % 2 == column % 2 { Color::Black } else { Color::White }
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
