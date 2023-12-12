use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::str::FromStr;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct CamelHand<C: Card, T: HandType<C>> {
    pub cards: [C; 5],
    pub hand_type: T,
}

pub trait HandType<C: Card>: Ord {
    fn from_cards(hand: &[C; 5]) -> Self;
}

pub trait Card: Ord + Hash + Sized {
    fn from_char(c: char) -> Result<Self>;
}

impl<C: Card, T: HandType<C>> FromStr for CamelHand<C, T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [C; 5] = s
            .chars()
            .map(C::from_char)
            .collect::<Result<Vec<C>>>()?
            .try_into()
            .ok()
            .context("String must have exaclty 5 cards")?;

        let hand_type = T::from_cards(&cards);

        Ok(CamelHand { cards, hand_type })
    }
}

impl<C: Card, T: HandType<C>> PartialEq for CamelHand<C, T> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl<C: Card, T: HandType<C>> Eq for CamelHand<C, T> {}

impl<C: Card, T: HandType<C>> Ord for CamelHand<C, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.cards.cmp(&other.cards))
    }
}

impl<C: Card, T: HandType<C>> PartialOrd for CamelHand<C, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_camel_cards<C: Card, T: HandType<C>>(
    input: &str,
) -> Result<BTreeMap<CamelHand<C, T>, u32>> {
    input.lines().map(parse_camel_hand).collect()
}

fn parse_camel_hand<C: Card, T: HandType<C>>(input: &str) -> Result<(CamelHand<C, T>, u32)> {
    let (hand, bid) = input
        .split_once(' ')
        .context("Hand and Bid must be space separated")?;

    let hand = hand.parse::<CamelHand<C, T>>()?;
    let bid = bid.parse::<u32>()?;

    Ok((hand, bid))
}
