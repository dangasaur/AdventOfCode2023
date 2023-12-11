use std::{
    ops::Range, 
    str::FromStr
};

use anyhow::{Context, Result};
use regex::Regex;
use strum_macros::EnumString;

use crate::aoc::{Solution, Solveable};

#[derive(Debug, PartialEq, EnumString)]
enum FoodCategory {
    #[strum(serialize = "seed")]
    Seed,
    #[strum(serialize = "soil")]
    Soil,
    #[strum(serialize = "fertilizer")]
    Fertilizer,
    #[strum(serialize = "water")]
    Water,
    #[strum(serialize = "light")]
    Light,
    #[strum(serialize = "temperature")]
    Temperature,
    #[strum(serialize = "humidity")]
    Humidity,
    #[strum(serialize = "location")]
    Location,
}

struct MapExpression {
    source_range: Range<i32>,
    destination_range: Range<i32>,
}
struct FoodCategoryMap {
    input: FoodCategory,
    output: FoodCategory,
    direct_maps: Vec<MapExpression>,
}

trait CategoryMapper {
    fn map_number (&self, number: i32) -> i32;
}

impl CategoryMapper for FoodCategoryMap {
    fn map_number (&self, number: i32) -> i32 {
        number + 1
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Day5 {}
impl Solveable for Day5 {
    fn solve(&self, lines: &Vec<String>) -> Result<Solution> {
        let seeds_pattern = Regex::new(r"^seeds:(.*)").context("invalid seeds line pattern")?;
        let map_header_pattern = Regex::new(r"(\w)-to-(\w)").context("invalid map header pattern")?;
        let map_pattern = Regex::new(r"(\d+)\s(\d+)\s(\d+)").context("invalid map pattern")?;
        
        let mut seeds: Vec<i32> = Vec::new();
        let mut input_maps: Vec<FoodCategoryMap> = Vec::new();

        for line in lines {
            if let Some(number_captures) = map_pattern.captures(line) {
                let (_, [destination_str, source_str, range_length_str]) = number_captures.extract();
                let destination = destination_str.parse::<i32>().context("failed to parse destination")?;
                let source = source_str.parse::<i32>().context("failed to parse source")?;
                let range_length = range_length_str.parse::<i32>().context("failed to parse range length")?;

                if let Some(cat) = input_maps.last_mut() {
                    cat.direct_maps.push(MapExpression { source_range: (source..source+range_length), destination_range: (destination..destination+range_length) })
                } else {
                    panic!("encountered map line before map header!");
                }
            } else if let Some(head_captures) = map_header_pattern.captures(line) {
                let (_, [input_str, output_str]) = head_captures.extract();
                let next_map = FoodCategoryMap {
                    input: FoodCategory::from_str(&input_str).context("unknown food category")?,
                    output: FoodCategory::from_str(&output_str).context("unknown food category")?,
                    direct_maps: Vec::new()
                };
                input_maps.push(next_map);

            } else if let Some(seed_captures) = seeds_pattern.captures(line) {
                let (_, [seed_numbers]) = seed_captures.extract();
                let mut seed_numbers: Vec<i32> = Regex::new(r"\d+").unwrap()
                    .find_iter(seed_numbers)
                    .map(|s| {
                        Ok(s.as_str().parse::<i32>().context("failed to parse seed number")?)
                    })
                    .collect::<Result<Vec<_>>>()?;
                seeds.append(&mut seed_numbers);
            }
        }

        Ok(Solution {
            part_1: "UNSOLVED".to_string(),
            part_2: "UNSOLVED".to_string(),
        })
    }
}