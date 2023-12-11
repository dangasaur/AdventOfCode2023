use std::collections::HashMap;

use anyhow::{Context, Result};
use regex::Regex;

use crate::aoc::{Solution, Solveable};

#[derive(Debug)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
    num_matches: i32,
    score: i32,
}

#[derive(Debug, Default, PartialEq)]
pub struct Day4 {}
impl Solveable for Day4 {
    fn solve(&self, lines: &Vec<String>) -> Result<Solution> {
        let number_pattern = Regex::new(r"\d+").context("invalid number pattern")?;

        let cards: Vec<Card> = lines
            .into_iter()
            .map(|line| {
                let (head, body) = line
                    .split_once(':')
                    .context("game line invalid, cannot split on :")?;
                let card_id = number_pattern
                    .find(head)
                    .context("failed to find card id")?
                    .as_str()
                    .parse::<i32>()
                    .context("failed to parse card id")?;

                let (win_s, my_s) = body.split_once("|").context("failed to split card body")?;
                let winning_numbers: Vec<i32> = number_pattern
                    .find_iter(win_s)
                    .map(|n| {
                        Ok(n.as_str()
                            .parse::<i32>()
                            .context("failed to parse winning number")?)
                    })
                    .collect::<Result<Vec<_>>>()?;
                let my_numbers: Vec<i32> = number_pattern
                    .find_iter(my_s)
                    .map(|n| {
                        Ok(n.as_str()
                            .parse::<i32>()
                            .context("failed to parse my number")?)
                    })
                    .collect::<Result<Vec<_>>>()?;

                let num_matches: i32 = winning_numbers
                    .iter()
                    .filter(|n| my_numbers.contains(n))
                    .collect::<Vec<_>>()
                    .len()
                    .try_into()
                    .unwrap();

                let mut score = 0;
                if num_matches > 0 {
                    score = 1;
                    for _ in 1..num_matches {
                        score = score * 2;
                    }
                }

                let card = Card {
                    id: card_id,
                    winning_numbers,
                    my_numbers,
                    num_matches,
                    score,
                };
                println!("{:?}", card);

                Ok(card)
            })
            .collect::<Result<Vec<_>>>()?;

        let part1_score: i32 = cards.iter().map(|c| c.score).sum();

        // initialize an id:count map with all positions having an initial count of 1 (the original cards)
        let mut card_counts: HashMap<i32, i32> =
            cards.iter().fold(HashMap::new(), |mut memo, next| {
                memo.insert(next.id, 1);
                memo
            });

        // for each card get the count and then increment the next n cards by the number of matching numbers, any expansion should have a count of zero
        for card in cards {
            let current_card_count = card_counts
                .get(&card.id)
                .context("card count not found")?
                .clone();
            println!(
                "card {} has {} copies x {} matchs",
                card.id, current_card_count, card.num_matches
            );

            if card.num_matches > 0 {
                let next_cards: Vec<i32> = (card.id + 1..card.id + card.num_matches + 1).collect();
                println!(
                    "\t adding {} copies to cards {:?}",
                    current_card_count,
                    next_cards,
                );
                for id in next_cards {
                    card_counts
                        .entry(id)
                        .and_modify(|count| *count += current_card_count)
                        .or_insert(0);
                }
            }
        }
        let total_num_cards: i32 = card_counts.values().sum();

        Ok(Solution {
            part_1: part1_score.to_string(),
            part_2: total_num_cards.to_string(),
        }) // part 1 23673
    }
}
