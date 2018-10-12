#![feature(box_syntax, box_patterns, duration_as_u128)]
extern crate ncurses;
extern crate rand;

mod mode;
use mode::height::Height;
use mode::quadrant::Quadrant;
use mode::{ModeState, Property, Empty};
use mode::column::ColumnOddness;

use mode::side::SideContainer;

mod color;
use color::Color;

use ncurses::*;
use std::time::{Duration, Instant};
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let mode = args.next().unwrap();

    initscr();
    printw("Welcome to chess trainer!\n");

    match &mode[..] {
        "oddness" => {
            ColumnOddness::print_help();
            cycle::<Empty, ColumnOddness>(Empty);
        },
        "color" => {
            Color::print_help();
            cycle::<Empty, Color>(Empty);
        },
        "quadrant" => {
            Quadrant::print_help();
            cycle::<SideContainer, Quadrant>(SideContainer::parse(args));
        },
        "height" => {
            Height::print_help();
            cycle::<Empty, Height>(Empty);
        },
        _ => {
            endwin();
            panic!("Unknown mode.")
        }
    };
}

fn cycle<State: ModeState, Prop: Property<State> + PartialEq>(mut state: State) {

    let mut total_time = Duration::new(0, 0);
    let mut total_answers = 0;
    let mut correct_answers = 0;

    loop {
        printw(&format!("{}", state));

        let row = rand::random::<u8>() % 8;
        let column = rand::random::<u8>() % 8;
        printw(&format!("{}{}? [",
            ('a' as u8 + column) as char,
            row + 1));

        let answer = Prop::calculate(&state, column, row);

        let time = Instant::now();
        let mut guess = None;
        while guess == None {
            guess = Prop::parse(std::char::from_u32(getch() as u32).unwrap());
        }

        let time = time.elapsed();
        total_answers += 1;
        total_time += time;

        let guess = guess.unwrap();

        let correct = if guess == answer {
            correct_answers += 1;
            "Yes!"
        } else {
            "No. "
        };

        let time  = format!("{:4}", time.as_millis());
        let speed = format!("{:.*}", 2, total_answers as f32 / (total_time.as_secs() as f32));
        let ratio = format!("{:.*}", 2, correct_answers as f32 / total_answers as f32);

        printw(&format!("]: {} Time of thinking: {}ms. Speed: {} answers/sec. Success ratio: {}\n",
            correct, time, speed, ratio));

        state.tick();

        refresh();
    }
}
