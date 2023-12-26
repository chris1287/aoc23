mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u32,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => {
            day1::part1();
            day1::part2();
        },
        2 => {
            day2::part1();
            day2::part2();
        },
        3 => {
            day3::part1();
            day3::part2();
        },
        4 => {
            day4::part1();
            day4::part2();
        },
        5 => {
            day5::part1();
            day5::part2();
        },
        6 => {
            day6::part1();
            day6::part2();
        },
        7 => {
            day7::part1();
            day7::part2();
        },
        8 => {
            day8::part1();
            day8::part2();
        },
        9 => {
            day9::part1();
            day9::part2();
        },
        10 => {
            day10::part1();
            day10::part2();
        },
        _ => {
            unimplemented!();
        }
    }
}

