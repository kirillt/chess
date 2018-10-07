#![feature(box_syntax, box_patterns)]
extern crate ncurses;
extern crate rand;

mod mode;
use mode::Property;
use mode::color::Color;
use mode::quadrant::Quadrant;
use mode::height::Height;

use ncurses::*;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    initscr();
    printw("Welcome to chess trainer!\n");

    let mode = args.next().unwrap();

    //let think_time = args
    //    .next().unwrap()
    //    .parse::<usize>().unwrap();

    //printw(&format!("Your think time is {}ms.\n", think_time));

    match &mode[..] {
        "color" => {
            Color::print_help();
            cycle::<Color>();
        },
        "quadrant" => {
            Quadrant::print_help();
            cycle::<Quadrant>();
        },
        "height" => {
            Height::print_help();
            cycle::<Height>();
        },
        _ => {
            endwin();
            panic!("Unknown mode.")
        }
    };
}

fn cycle<Prop: Property + PartialEq>() {
    loop {
        let row = rand::random::<u8>() % 8;
        let column = rand::random::<u8>() % 8;
        printw(&format!("{}{}? [",
            ('a' as u8 + column) as char,
            row + 1));

        let answer = Prop::calculate(column, row);

        let mut guess = None;
        while guess == None {
            guess = Prop::parse(std::char::from_u32(getch() as u32).unwrap());
        }

        let guess = guess.unwrap();

        if guess == answer {
            printw("]: Yes!\n");
        } else {
            printw("]: No.\n");
        }

        refresh();
    }
}