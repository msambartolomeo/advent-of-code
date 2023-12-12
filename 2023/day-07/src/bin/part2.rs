use day_07::{Card, HandType};

use anyhow::{bail, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let hands = day_07::parse_camel_cards::<CamelCardJoker, CamelHandTypeJoker>(input)?;

    let result = hands
        .into_values()
        .enumerate()
        .map(|(rank, bid)| (rank as u64 + 1) * bid as u64)
        .sum();

    Ok(result)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CamelCardJoker {
    J,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CamelHandTypeJoker {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card for CamelCardJoker {
    fn from_char(c: char) -> Result<Self> {
        Ok(match c {
            '2' => CamelCardJoker::C2,
            '3' => CamelCardJoker::C3,
            '4' => CamelCardJoker::C4,
            '5' => CamelCardJoker::C5,
            '6' => CamelCardJoker::C6,
            '7' => CamelCardJoker::C7,
            '8' => CamelCardJoker::C8,
            '9' => CamelCardJoker::C9,
            'T' => CamelCardJoker::T,
            'J' => CamelCardJoker::J,
            'Q' => CamelCardJoker::Q,
            'K' => CamelCardJoker::K,
            'A' => CamelCardJoker::A,
            _ => bail!("Character is not a card"),
        })
    }
}

impl HandType<CamelCardJoker> for CamelHandTypeJoker {
    fn from_cards(hand: &[CamelCardJoker; 5]) -> Self {
        let mut counts = hand.iter().counts();

        let joker_count = counts.remove(&CamelCardJoker::J);
        let mut counts = counts.values().sorted().copied().collect_vec();

        if let Some(n) = joker_count {
            if n == 5 {
                counts.push(n);
            } else {
                counts.last_mut().map(|m| *m += n);
            }
        }

        match counts[..] {
            [5] => CamelHandTypeJoker::FiveOfAKind,
            [1, 4] => CamelHandTypeJoker::FourOfAKind,
            [2, 3] => CamelHandTypeJoker::FullHouse,
            [1, 1, 3] => CamelHandTypeJoker::ThreeOfAKind,
            [1, 2, 2] => CamelHandTypeJoker::TwoPair,
            [1, 1, 1, 2] => CamelHandTypeJoker::OnePair,
            [1, 1, 1, 1, 1] => CamelHandTypeJoker::HighCard,
            _ => unreachable!("Unexpected array {counts:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = process(input).unwrap();

        assert_eq!(5905, result);
    }
}
