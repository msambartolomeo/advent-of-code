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
    let hands = day_07::parse_camel_cards::<CamelCard, CamelHandType>(input)?;

    let result = hands
        .into_values()
        .enumerate()
        .map(|(rank, bid)| (rank as u64 + 1) * bid as u64)
        .sum();

    Ok(result)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CamelCard {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CamelHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card for CamelCard {
    fn from_char(c: char) -> Result<Self> {
        Ok(match c {
            '2' => CamelCard::C2,
            '3' => CamelCard::C3,
            '4' => CamelCard::C4,
            '5' => CamelCard::C5,
            '6' => CamelCard::C6,
            '7' => CamelCard::C7,
            '8' => CamelCard::C8,
            '9' => CamelCard::C9,
            'T' => CamelCard::T,
            'J' => CamelCard::J,
            'Q' => CamelCard::Q,
            'K' => CamelCard::K,
            'A' => CamelCard::A,
            _ => bail!("Character is not a card"),
        })
    }
}

impl HandType<CamelCard> for CamelHandType {
    fn from_cards(hand: &[CamelCard; 5]) -> Self {
        let counts = hand.iter().counts();
        let counts = counts.values().sorted().collect_vec();

        match counts[..] {
            [5] => CamelHandType::FiveOfAKind,
            [1, 4] => CamelHandType::FourOfAKind,
            [2, 3] => CamelHandType::FullHouse,
            [1, 1, 3] => CamelHandType::ThreeOfAKind,
            [1, 2, 2] => CamelHandType::TwoPair,
            [1, 1, 1, 2] => CamelHandType::OnePair,
            [1, 1, 1, 1, 1] => CamelHandType::HighCard,
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

        assert_eq!(6440, result);
    }
}
