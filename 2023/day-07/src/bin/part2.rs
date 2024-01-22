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
        .map(|(rank, bid)| (rank as u64 + 1) * u64::from(bid))
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
            '2' => Self::C2,
            '3' => Self::C3,
            '4' => Self::C4,
            '5' => Self::C5,
            '6' => Self::C6,
            '7' => Self::C7,
            '8' => Self::C8,
            '9' => Self::C9,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => bail!("Character is not a card"),
        })
    }
}

impl HandType<CamelCardJoker> for CamelHandTypeJoker {
    fn from_cards(hand: &[CamelCardJoker; 5]) -> Self {
        let mut counts = hand.iter().counts();

        let joker_count = counts.remove(&CamelCardJoker::J);
        let mut counts = counts.values().sorted().rev().copied().collect_vec();

        match joker_count {
            Some(5) => counts.push(5),
            Some(n) => counts[0] += n,
            _ => (),
        }

        match counts[..] {
            [5] => Self::FiveOfAKind,
            [4, 1] => Self::FourOfAKind,
            [3, 2] => Self::FullHouse,
            [3, 1, 1] => Self::ThreeOfAKind,
            [2, 2, 1] => Self::TwoPair,
            [2, 1, 1, 1] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
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
