#![feature(box_syntax, box_patterns, duration_as_u128)]
extern crate ncurses;
extern crate rand;

mod mode;
use mode::Color;
use mode::Property;
use mode::quadrant::Quadrant;
use mode::height::Height;
use mode::column::ColumnOddness;

use ncurses::*;
use std::time::{Duration, Instant};
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    initscr();
    printw("Welcome to chess trainer!\n");

    let mode = args.next().unwrap();

    let submode = args.next();
    let side = submode.clone()
        .and_then(|input| Color::parse_str(input));
    let period = if side.is_none() {
        submode.and_then(|input| input.parse::<u8>().ok())
            .unwrap_or(32)
    } else {
        std::u8::MAX
    };

    match &mode[..] {
        "oddness" => {
            ColumnOddness::print_help();
            cycle::<ColumnOddness>(side, period);
        },
        "color" => {
            Color::print_help();
            cycle::<Color>(side, period);
        },
        "quadrant" => {
            Quadrant::print_help();
            cycle::<Quadrant>(side, period);
        },
        "height" => {
            Height::print_help();
            cycle::<Height>(side, period);
        },
        _ => {
            endwin();
            panic!("Unknown mode.")
        }
    };
}

fn cycle<Prop: Property + PartialEq>(side: Option<Color>, period: u8) {
    let mut total_time = Duration::new(0, 0);
    let mut total_answers = 0;
    let mut correct_answers = 0;

    let mut side = side.unwrap_or(Color::White);
    let mut countdown = period;

    loop {
        if countdown == 0 {
            side = Color::invert(side);
            countdown = period;
        }

        let row = rand::random::<u8>() % 8;
        let column = rand::random::<u8>() % 8;
        printw(&format!("<{}> {}{}? [",
            side.print(),
            ('a' as u8 + column) as char,
            row + 1));

        let answer = Prop::calculate(&side, column, row);

        let time = Instant::now();
        let mut guess = None;
        while guess == None {
            guess = Prop::parse(std::char::from_u32(getch() as u32).unwrap());
        }

        let time = time.elapsed();
        total_answers += 1;
        total_time += time;

        let time  = format!("{:4}", time.as_millis());
        let speed = format!("{:.*}", 2, total_answers as f32 / (total_time.as_secs() as f32));
        let ratio = format!("{:.*}", 2, correct_answers as f32 / total_answers as f32);

        let guess = guess.unwrap();

        let correct = if guess == answer {
            correct_answers += 1;
            "Yes!"
        } else {
            "No. "
        };

        printw(&format!("]: {} Time of thinking: {}ms. Speed: {} answers/sec. Success ratio: {}\n",
            correct, time, speed, ratio));

        countdown -= 1;
        refresh();
    }
}
