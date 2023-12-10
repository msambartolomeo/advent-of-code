use std::cmp::{Ordering, Reverse};
use std::collections::BTreeMap;
use std::str::FromStr;

use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct CamelHand {
    pub cards: [CamelCard; 5],
    pub hand_type: CamelHandType,
}

impl FromStr for CamelHand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [CamelCard; 5] = s
            .chars()
            .map(TryInto::<CamelCard>::try_into)
            .collect::<Result<Vec<CamelCard>>>()?
            .try_into()
            .ok()
            .context("String must have exaclty 5 cards")?;

        let counts = cards.iter().counts();
        let counts = counts.values().sorted_by_key(|&n| Reverse(n)).collect_vec();

        let hand_type = match counts[..] {
            [5] => CamelHandType::FiveOfAKind,
            [4, 1] => CamelHandType::FourOfAKind,
            [3, 2] => CamelHandType::FullHouse,
            [3, 1, 1] => CamelHandType::ThreeOfAKind,
            [2, 2, 1] => CamelHandType::TwoPair,
            [2, 1, 1, 1] => CamelHandType::OnePair,
            [1, 1, 1, 1, 1] => CamelHandType::HighCard,
            _ => unreachable!("Unexpected array {counts:?}",),
        };

        Ok(CamelHand { cards, hand_type })
    }
}

impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for CamelHand {}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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

impl TryFrom<char> for CamelCard {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
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

pub fn parse_camel_cards(input: &str) -> Result<BTreeMap<CamelHand, u32>> {
    input.lines().map(parse_camel_hand).collect()
}

fn parse_camel_hand(input: &str) -> Result<(CamelHand, u32)> {
    let (hand, bid) = input
        .split_once(' ')
        .context("Hand and Bid must be space separated")?;

    let hand = hand.parse::<CamelHand>()?;
    let bid = bid.parse::<u32>()?;

    Ok((hand, bid))
}
