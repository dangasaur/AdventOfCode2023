use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};
use strum_macros::EnumString;

use aoc23_rust::day1::Day1;
use aoc23_rust::day2::Day2;
use aoc23_rust::day3::Day3;
use aoc23_rust::day4::Day4;
use aoc23_rust::day5::Day5;
use aoc23_rust::aoc::Solveable;

#[derive(Debug, PartialEq, EnumString)]
pub enum Day {
    #[strum(serialize = "day1", serialize = "1")]
    D1(Day1),

    #[strum(serialize = "day2", serialize = "2")]
    D2(Day2),

    #[strum(serialize = "day3", serialize = "3")]
    D3(Day3),
    
    #[strum(serialize = "day4", serialize = "4")]
    D4(Day4),

    #[strum(serialize = "day5", serialize = "5")]
    D4(Day5),
}

impl Solveable for Day {
    fn solve (&self, lines: &Vec<String>) -> Result<aoc23_rust::aoc::Solution> {
        match self {
            Day::D1(inner) => inner.solve(lines),
            Day::D2(inner) => inner.solve(lines),
            Day::D3(inner) => inner.solve(lines),
            Day::D4(inner) => inner.solve(lines),
            Day::D5(inner) => inner.solve(lines),
        }
    }
}

async fn read_input(input_path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.context("failed to parse line"))
        .collect()
}

#[derive(Parser, Default, Debug)]
struct Arguments {
    day: String,
    input_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();
    let solver = Day::from_str(&args.day).context("unknown day lol")?;
    let lines = read_input(Path::new(&args.input_file)).await.context("failed to get lines")?;

    let solution = solver.solve(&lines).expect("failed to solve");

    println!("part 1 answer: {}", solution.part_1);
    println!("part 2 answer: {}", solution.part_2);
    Ok(())
}
