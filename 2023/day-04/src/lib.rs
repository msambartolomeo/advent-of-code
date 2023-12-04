use std::collections::HashSet;

use anyhow::{Context, Result};

pub struct ScratchCard {
    pub id: u32,
    pub winning_numbers: HashSet<u32>,
    pub numbers_you_have: HashSet<u32>,
}

impl ScratchCard {
    #[must_use]
    pub fn winning_count(&self) -> usize {
        self.numbers_you_have
            .intersection(&self.winning_numbers)
            .count()
    }
}

/// Parses an of cards
///
/// Example input:
/// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// ...
///
/// # Errors
/// Returns Err if the input is invalid
pub fn parse_cards(input: &str) -> impl Iterator<Item = Result<ScratchCard>> + '_ {
    input.lines().map(parse_card)
}

fn parse_card(input: &str) -> Result<ScratchCard> {
    let (id, numbers) = input.split_once(':').context("Input must have :")?;

    let id = id
        .split_whitespace()
        .nth(1)
        .context("Id must be of format Card n, skipping Card")?;

    let id = id.parse::<u32>()?;

    let (winning_numbers, numbers_you_have) = numbers
        .split_once('|')
        .context("Numbers must be separated by |")?;

    let winning_numbers = to_numbers(winning_numbers)?;
    let numbers_you_have = to_numbers(numbers_you_have)?;

    Ok(ScratchCard {
        id,
        winning_numbers,
        numbers_you_have,
    })
}

#[inline]
fn to_numbers(string: &str) -> Result<HashSet<u32>> {
    string.split_whitespace().map(|s| Ok(s.parse()?)).collect()
}
