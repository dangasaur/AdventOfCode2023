use anyhow::Result;

#[derive(Debug)]
pub struct Solution {
    pub part_1: String,
    pub part_2: String,
}

pub trait Solveable {
    fn solve (&self, lines: &Vec<String>) -> Result<Solution>;
}
