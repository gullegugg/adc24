use std::{error::Error, io::stdin};

use adc24::{solve, Part};
use clap::Parser;

#[derive(Parser)]
struct MainArgs {
    day: u8,
    part: Part,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = MainArgs::parse();
    let input = stdin().lines().collect::<Result<Vec<String>, _>>()?;
    let res = solve(args.day, args.part, input)?;
    println!("{res}");
    Ok(())
}
