use anyhow::{Context, Result};
use clap::{Parser};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::collections::HashMap;
use std::ops::Deref;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    input_file: String,
}

async fn read_input(input_path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.context("failed to parse line"))
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let input_file = cli.input_file.deref();

    let lines = read_input(Path::new(input_file)).await.context("failed to get lines")?;
    println!("number of lines: {}", lines.len());

    let re_numbers = Regex::new(r"\d").unwrap();

    let sum_phase1 = lines.iter().fold(0, |memo, next| {
        let str_numbers: Vec<&str> = re_numbers.find_iter(next).map(|n| n.as_str()).collect();
        format!("{}{}", str_numbers.first().unwrap(), str_numbers.last().unwrap()).parse::<i32>().unwrap() + memo
    });

    println!("phase 1 result: {}", sum_phase1);

    // phase 2
    let number_map = HashMap::from([
        ("0", 0), ("zero", 0),
        ("1", 1), ("one", 1),
        ("2", 2), ("two", 2),
        ("3", 3), ("three", 3),
        ("4", 4), ("four", 4),
        ("5", 5), ("five", 5),
        ("6", 6), ("six", 6),
        ("7", 7), ("seven", 7),
        ("8", 8), ("eight", 8),
        ("9", 9), ("nine", 9)
    ]);

    // let re_numbers_spelled_out = RegexSet::new([
    //     r"\d",
    //     r"one",
    //     r"two",
    //     r"three",
    //     r"four",
    //     r"five",
    //     r
    // ]).unwrap();

    let re_numbers_spelled_out = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_numbers_spelled_out_r = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let sum_phase2 = lines.iter().fold(0, |memo, next| {
        let str_numbers_forward: Vec<&str> = re_numbers_spelled_out.find_iter(next).map(|n| n.as_str()).collect();

        let next_reversed: String = next.chars().rev().collect();
        let str_numbers_reverse: Vec<&str> = re_numbers_spelled_out_r.find_iter(next_reversed.as_str()).map(|n| n.as_str()).collect();

        let first = str_numbers_forward.first().unwrap();
        let last = str_numbers_reverse.first().unwrap();
        println!("first: {}, last: {}", first, last);

        let combined = format!("{}{}", number_map.get(first).unwrap(), number_map.get(last).unwrap()).parse::<i32>().unwrap();
        println!("{} -- {}", combined, next);
        combined + memo
    });

    println!("phase 2 result: {}", sum_phase2);

    Ok(())
}
