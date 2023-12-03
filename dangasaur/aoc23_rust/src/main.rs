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

use crate::aoc::Solution;
use crate::day1::solve;

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

    let solution = solve(&lines).await;

    Ok(())
}
