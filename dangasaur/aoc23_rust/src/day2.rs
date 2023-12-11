use anyhow::{Result, Context};
use regex::Regex;

use crate::aoc::{Solution, Solveable};

#[derive(Debug)]
struct Roll {
    roll_string: String,
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>,
}

#[derive(Debug)]
struct Game {
    game_string: String,
    rolls: Vec<Roll>,
    id: i32,
    max_red: i32,
    max_green: i32,
    max_blue: i32,
}

#[derive(Debug, Default, PartialEq)]
pub struct Day2 {}
impl Solveable for Day2 {
    fn solve (&self, lines: &Vec<String>) -> Result<Solution> {
        let game_id_pattern = Regex::new(r"\d+").context("invalid game ID regular expression")?;

        let mut games: Vec<Game> = Vec::new();
        
        // split the line by : to get the game number and the rolls string
        for line in lines {
            let (right, left) = line.split_once(":").context("game line invalid, cannot split on :")?;
            let game_id = game_id_pattern.find(right).context("failed to find game ID in line")?
                .as_str().parse::<i32>().context("failed to parse game ID to integer")?;

            let rolls = left.split(";")
                .map(|roll_string| {
                    let mut roll = Roll { 
                        roll_string: roll_string.to_string(),
                        red: None,
                        green: None,
                        blue: None,
                    };

                    let rgb_pattern = Regex::new(r"(\d+)\s(red|green|blue)").context("invalid rgb regular expression")?;
                    for (_, [number, color]) in rgb_pattern.captures_iter(roll_string).map(|c| c.extract()) {
                        let number_i32 = number.parse::<i32>().context("failed to parse roll number")?; // I want this to be a ? instead but there's an issue w/ doing it in the closure...
                        match color {
                            "red" => roll.red = Some(number_i32),
                            "green" => roll.green = Some(number_i32),
                            "blue" => roll.blue = Some(number_i32),
                            _ => panic!("could not serialize roll into colors red, green, or blue")
                        }
                    }
                    
                    println!("got rolls: {:?}, {:?}, {:?} from roll string: {}", roll.red, roll.blue, roll.green, roll.roll_string);
                    Ok(roll)
                })
                .collect::<Result<Vec<_>>>()?;
            
            // find the maximum and minimum rolls
            let mut rgb_max = (0, 0, 0);
            for roll in &rolls {
                if let Some(red) = roll.red { 
                    if red > rgb_max.0 { rgb_max.0 = red }
                }

                if let Some(green) = roll.green { 
                    if green > rgb_max.1 { rgb_max.1 = green }
                }

                if let Some(blue) = roll.blue { 
                    if blue > rgb_max.2 { rgb_max.2 = blue }
                }
            }

            let game = Game {
                game_string: line.clone(),
                rolls: rolls,
                id: game_id,
                max_red: rgb_max.0,
                max_green: rgb_max.1,
                max_blue: rgb_max.2,
            };
            println!("game: {} has max rolls: {}, {}, {}", game.id, game.max_red, game.max_green, game.max_blue);
            games.push(game)
        }

        // part1: identify which games are possible based on provided max rolls
        let possible_games_sum: i32 = games.iter()
            .filter_map(|g| {
                if g.max_red <= 12 && g.max_green <= 13 && g.max_blue <=14 { 
                    Some(g.id) 
                } else {
                    None
                }
            })
            .sum();

        // part2: identify the minimum games
        let minimum_cubes_sum: i32 = games.iter()
            .map(|g| {
                g.max_red * g.max_green * g.max_blue
            })
            .sum();

        Ok(Solution { part_1: possible_games_sum.to_string(), part_2: minimum_cubes_sum.to_string() })
    }
}