use anyhow::{bail, Result};
use itertools::Itertools;

use crate::{Letter, WordSearch};

pub fn parse(input: &str) -> Result<WordSearch> {
    let word_search = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().map(Letter::try_from).collect())
        .collect::<Result<Vec<Vec<Letter>>>>()?;

    if !word_search.iter().map(Vec::len).all_equal() {
        bail!("Invalid size")
    }

    Ok(WordSearch(word_search))
}
