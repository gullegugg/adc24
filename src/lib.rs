use std::error::Error;

use clap::ValueEnum;

pub mod day1;

#[derive(Clone, ValueEnum)]
pub enum Part {
    First,
    Second,
}

pub fn solve(day: u8, part: Part, input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    match day {
        1 => day1::solve(part, input),
        invalid => Err(format!("Day not implemented: {invalid}").into()),
    }
}
