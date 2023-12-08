use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use anyhow::{bail, Context, Result};

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

        let mut counts = HashMap::with_capacity(5);

        for card in cards {
            counts.entry(card).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        let hand_type = match counts[0] {
            (card, 5) => CamelHandType::FiveOfAKind(card),
            (card, 4) => CamelHandType::FourOfAKind(card),
            (card, 3) => match counts[1] {
                (other, 2) => CamelHandType::FullHouse(card, other),
                (_, _) => CamelHandType::ThreeOfAKind(card),
            },
            (card, 2) => match counts[1] {
                (other, 2) => CamelHandType::TwoPair(card, other),
                (_, _) => CamelHandType::OnePair(card),
            },
            (card, 1) => CamelHandType::HighCard(card),
            _ => unreachable!("number can only be 5"),
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

#[derive(Debug, PartialEq, Eq)]
pub enum CamelHandType {
    HighCard(CamelCard),
    OnePair(CamelCard),
    TwoPair(CamelCard, CamelCard),
    ThreeOfAKind(CamelCard),
    FullHouse(CamelCard, CamelCard),
    FourOfAKind(CamelCard),
    FiveOfAKind(CamelCard),
}

impl CamelHandType {
    fn as_usize(&self) -> usize {
        match self {
            CamelHandType::HighCard(_) => 0,
            CamelHandType::OnePair(_) => 1,
            CamelHandType::TwoPair(_, _) => 2,
            CamelHandType::ThreeOfAKind(_) => 3,
            CamelHandType::FullHouse(_, _) => 4,
            CamelHandType::FourOfAKind(_) => 5,
            CamelHandType::FiveOfAKind(_) => 6,
        }
    }
}

impl PartialOrd for CamelHandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelHandType {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.as_usize()).cmp(&other.as_usize())
    }
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
