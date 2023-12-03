use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::collections::HashMap;
use std::ops::Deref;
use regex::{Regex, Match};

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

    let patterns = vec![
        Regex::new(r"\d").unwrap(),
        Regex::new(r"one").unwrap(),
        Regex::new(r"two").unwrap(),
        Regex::new(r"three").unwrap(),
        Regex::new(r"four").unwrap(),
        Regex::new(r"five").unwrap(),
        Regex::new(r"six").unwrap(),
        Regex::new(r"seven").unwrap(),
        Regex::new(r"eight").unwrap(),
        Regex::new(r"nine").unwrap(),
    ];

    let sum_phase2 = lines.iter().fold(0, |memo, next| {
        // for each regular expression get matches
        let matches: Vec<Match> = patterns.iter()
            .map(|p| p.find_iter(next).collect::<Vec<Match>>())
            .flatten()
            .collect();

        let first_match = matches.iter().reduce(|memo, next| if next.start() < memo.start() { next } else { memo }).unwrap().as_str();
        let last_match = matches.iter().reduce(|memo, next| if next.start() > memo.start() { next } else { memo }).unwrap().as_str();

        println!("first: {}, last: {}", first_match, last_match);

        let combined = format!("{}{}", number_map.get(first_match).unwrap(), number_map.get(last_match).unwrap()).parse::<i32>().unwrap();
        println!("{} -- {}", combined, next);
        combined + memo
    });

    println!("phase 2 result: {}", sum_phase2);

    Ok(())
}
