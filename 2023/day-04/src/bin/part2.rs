use std::collections::VecDeque;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let cards = day_04::parse_cards(input);

    let (result, _) = cards.process_results(|it| {
        it.fold((0, VecDeque::new()), |(acc, mut card_counts), c| {
            let current_count = card_counts.pop_front().unwrap_or(1);
            let len = card_counts.len();
            let winning_count = c.winning_count();

            card_counts
                .iter_mut()
                .take(winning_count)
                .for_each(|q| *q += current_count);

            if len <= winning_count {
                card_counts.extend(std::iter::repeat(1 + current_count).take(winning_count - len))
            }

            (acc + current_count, card_counts)
        })
    })?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = process(input).unwrap();

        assert_eq!(30, result);
    }
}
