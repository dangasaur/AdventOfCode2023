use anyhow::{Result, Context};
use regex::Regex;

use crate::aoc::{Solution, Solveable};

#[derive(Debug)]
struct SchematicPart {
    start_pos: usize,
    end_pos: usize,
    value: i32,
}

#[derive(Debug)]
struct SchematicLine {
    numbers: Vec<SchematicPart>,
    symbols: Vec<usize>,
    gear_symbols: Vec<usize>,
}

#[derive(Debug)]
struct Window<'a> {
    behind: Option<&'a SchematicLine>,
    current: Option<&'a SchematicLine>,
    ahead: Option<&'a SchematicLine>,
}

fn is_adjacent(symbol_pos: &usize, num_start_pos: &usize, num_end_pos: &usize) -> bool {
    // this prevents issues if the match start position is 0
    let start_floor: usize = if num_start_pos.clone() == usize::MIN { 0 } else { num_start_pos - 1 };
    symbol_pos >= &start_floor && symbol_pos <= &num_end_pos
}

fn part_1(schematic_lines: &Vec<SchematicLine>) -> i32 {
        let mut parts: Vec<i32> = Vec::new();
        let mut window = Window {
            behind: None,
            current: None,
            ahead: None,
        };

        let mut schematic_line_iter = schematic_lines.iter();

        // load the first two lines into the window to get started
        window.current = schematic_line_iter.next();
        window.ahead = schematic_line_iter.next();

        loop {
            // if the window has a current line, identify parts
            if let Some(current_line) = window.current {
                // for each number determine frame then search for any symbols in frame, 0 indexing
                let identified_parts: Vec<i32> = current_line.numbers.iter().filter_map(|n| {
                    let mut found_symbol = current_line.symbols.iter().find(|s| { is_adjacent(s, &n.start_pos, &n.end_pos)});
                    
                    if found_symbol.is_none() {
                        if let Some(behind_line) = window.behind {
                            found_symbol = behind_line.symbols.iter().find(|s| { is_adjacent(s, &n.start_pos, &n.end_pos)});
                        }
                    }

                    if found_symbol.is_none() {
                        if let Some(ahead_line) = window.ahead {
                            found_symbol = ahead_line.symbols.iter().find(|s| { is_adjacent(s, &n.start_pos, &n.end_pos)});
                        }
                    }

                    match found_symbol {
                        Some(_) => Some(n.value),
                        None => None
                    }
                }).collect();

                for p in identified_parts {
                    parts.push(p);
                }

                // shift the window "down"
                window.behind = window.current;
                window.current = window.ahead;
                window.ahead = schematic_line_iter.next();
            } else {
                break;
            }
        } 

        parts.into_iter().sum()
}

fn part_2(schematic_lines: &Vec<SchematicLine>) -> i32 {
        let mut gears: Vec<i32> = Vec::new();
        let mut window = Window {
            behind: None,
            current: None,
            ahead: None,
        };

        let mut schematic_line_iter = schematic_lines.iter();

        // load the first two lines into the window to get started
        window.current = schematic_line_iter.next();
        window.ahead = schematic_line_iter.next();

        loop {
            // if the window has a current line, identify any gears
            if let Some(current_line) = window.current {
                if current_line.gear_symbols.len() > 0 {
                    // for each gear symbol on the line, check all lines for adjacent numbers, if there are only two it's a gear
                    let line_gears: Vec<i32> = current_line.gear_symbols.iter()
                        .filter_map(|gs| {
                            let mut numbers_found: Vec<i32> = Vec::new();
                            let mut current_line_numbers: Vec<i32> = current_line.numbers.iter().filter_map(|n| {
                                if is_adjacent(gs, &n.start_pos, &n.end_pos) {
                                    Some(n.value.clone())
                                } else {
                                    None
                                }
                            }).collect();
                            numbers_found.append(&mut current_line_numbers);

                            
                            if let Some(behind_line) = window.behind {
                                let mut behind_line_numbers: Vec<i32> = behind_line.numbers.iter().filter_map(|n| {
                                    if is_adjacent(gs, &n.start_pos, &n.end_pos) {
                                        Some(n.value.clone())
                                    } else {
                                        None
                                    }
                                }).collect();
                                numbers_found.append(&mut behind_line_numbers);
                            }

                            if let Some(ahead_line) = window.ahead {
                                let mut ahead_line_numbers: Vec<i32> = ahead_line.numbers.iter().filter_map(|n| {
                                    if is_adjacent(gs, &n.start_pos, &n.end_pos) {
                                        Some(n.value.clone())
                                    } else {
                                        None
                                    }
                                }).collect();
                                numbers_found.append(&mut ahead_line_numbers);
                            }

                            if numbers_found.len() == 2 { 
                                Some(numbers_found.into_iter().product())
                            } else {
                                None
                            }
                        })
                        .collect();

                    for g in line_gears {
                        gears.push(g)
                    }
                }

                // shift window
                window.behind = window.current;
                window.current = window.ahead;
                window.ahead = schematic_line_iter.next();
            } else {
                break;
            }
        } 

        gears.into_iter().sum()
}

#[derive(Debug, Default, PartialEq)]
pub struct Day3 {}
impl Solveable for Day3 {
    fn solve (&self, lines: &Vec<String>) -> Result<Solution> {
        let re_number = Regex::new(r"\d+").context("invalide number regex pattern")?;
        let re_symbol = Regex::new(r"[^\d.]").context("invalid symbol regex pattern")?;
        let re_gear_symbol = Regex::new(r"[*]").context("invalid gear symbol regex pattern")?;

        let schematic_lines: Vec<SchematicLine> = lines.into_iter().map(|line| {
            let numbers = re_number.find_iter(line)
                .map(|m| {
                    Ok(SchematicPart { 
                        start_pos: m.start(), 
                        end_pos: m.end(), 
                        value: m.as_str().parse::<i32>().context("failed to parse value for part")?,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            
            let symbols: Vec<usize> = re_symbol.find_iter(line)
                .map(|m| {
                    m.start()
                })
                .collect();

            let gear_symbols: Vec<usize> = re_gear_symbol.find_iter(line)
                .map(|m| {
                    m.start()
                })
                .collect();

            let schematic_line = SchematicLine { 
                numbers,
                symbols,
                gear_symbols,
            };
            Ok(schematic_line)
        })
        .collect::<Result<Vec<_>>>()?;

        let parts_sum = part_1(&schematic_lines);
        let gears_sum = part_2(&schematic_lines);
        Ok(Solution { part_1: parts_sum.to_string(), part_2: gears_sum.to_string() })
    }
}
