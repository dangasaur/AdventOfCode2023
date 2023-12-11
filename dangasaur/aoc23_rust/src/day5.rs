use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;
use strum_macros::{Display, EnumString};

use crate::aoc::{Solution, Solveable};

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
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
    source_start: i32,
    destination_start: i32,
    range_length: i32,
}
struct FoodCategoryMap {
    input: FoodCategory,
    output: FoodCategory,
    direct_maps: Vec<MapExpression>,
}

trait CategoryMapper {
    fn map_number (&self, number: &i32) -> i32;
}

impl CategoryMapper for FoodCategoryMap {
    fn map_number (&self, number: &i32) -> i32 {
        // logic: if the number is covered by any direct map source range, use the position of the number in that range to determine the output, else return the number
        let direct_map = self.direct_maps.iter().find(|dm| {
            *number >= dm.source_start && *number <= dm.source_start + dm.range_length
        });
        match direct_map {
            Some(dm) => { 
                dm.destination_start + (*number - dm.source_start)
            },
            None => number.clone()
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Day5 {}
impl Solveable for Day5 {
    fn solve(&self, lines: &Vec<String>) -> Result<Solution> {
        let seeds_pattern = Regex::new(r"^seeds:(.*)").context("invalid seeds line pattern")?;
        let map_header_pattern = Regex::new(r"(\w+)-to-(\w+)").context("invalid map header pattern")?;
        let map_pattern = Regex::new(r"^(\d+)\s(\d+)\s(\d+)$").context("invalid map pattern")?;
        
        let start_category = FoodCategory::Seed;
        let final_category = FoodCategory::Location;
        
        let mut seeds: Vec<i32> = Vec::new();
        let mut input_maps: Vec<FoodCategoryMap> = Vec::new();

        for line in lines {
            if let Some(number_captures) = map_pattern.captures(line) {
                let (_, [destination_str, source_str, range_length_str]) = number_captures.extract();
                let destination_start = destination_str.parse::<i32>().context("failed to parse destination")?;
                let source_start = source_str.parse::<i32>().context("failed to parse source")?;
                let range_length = range_length_str.parse::<i32>().context("failed to parse range length")?;

                if let Some(cat) = input_maps.last_mut() {
                    cat.direct_maps.push(MapExpression { source_start, destination_start, range_length })
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
        
        for seed in seeds {
            let mut map_history: Vec<String> = Vec::new();
            let mut next_input_value = seed.clone();
            let mut next_input_category = &start_category;

            while next_input_category != &final_category {
                let mapper = input_maps.iter()
                    .find(|f| f.input == *next_input_category).context("no food category map found for next category")?;
                let output_value = mapper.map_number(&next_input_value);
                map_history.push(format!("{} {} => {} {}", next_input_category, next_input_value, output_value, mapper.output));

                next_input_value = output_value;
                next_input_category = &mapper.output;
            }
            
            println!("seed: {} map history: {:?}", seed, map_history);
        }

        Ok(Solution {
            part_1: "UNSOLVED".to_string(),
            part_2: "UNSOLVED".to_string(),
        })
    }
}