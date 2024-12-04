use std::ops::Add;

use anyhow::bail;

pub mod parser;

pub struct WordSearch(Vec<Vec<Letter>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Letter {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Letter {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'X' => Ok(Self::X),
            'M' => Ok(Self::M),
            'A' => Ok(Self::A),
            'S' => Ok(Self::S),
            l => bail!("Invalid Letter: {l}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct I(isize, isize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U(pub usize, pub usize);

impl From<(usize, usize)> for U {
    fn from((x, y): (usize, usize)) -> Self {
        Self(x, y)
    }
}

impl From<(isize, isize)> for I {
    fn from((x, y): (isize, isize)) -> Self {
        Self(x, y)
    }
}

impl Add<I> for U {
    type Output = Option<Self>;

    fn add(self, rhs: I) -> Self::Output {
        Some(Self(
            self.0.checked_add_signed(rhs.0)?,
            self.1.checked_add_signed(rhs.1)?,
        ))
    }
}

const SEARCH_OPTIONS: [[I; 3]; 8] = [
    [I(0, 1), I(0, 2), I(0, 3)],
    [I(0, -1), I(0, -2), I(0, -3)],
    [I(1, 0), I(2, 0), I(3, 0)],
    [I(-1, 0), I(-2, 0), I(-3, 0)],
    [I(1, 1), I(2, 2), I(3, 3)],
    [I(-1, -1), I(-2, -2), I(-3, -3)],
    [I(1, -1), I(2, -2), I(3, -3)],
    [I(-1, 1), I(-2, 2), I(-3, 3)],
];

const XMAS: [Letter; 4] = [Letter::X, Letter::M, Letter::A, Letter::S];

impl WordSearch {
    #[must_use]
    pub fn size(&self) -> U {
        U(self.0.len(), self.0[0].len())
    }

    #[must_use]
    pub fn get(&self, index: U) -> Option<Letter> {
        self.0.get(index.0).and_then(|v| v.get(index.1)).copied()
    }

    #[must_use]
    pub fn search_xmas(&self, index: U) -> u64 {
        if self.get(index) != Some(XMAS[0]) {
            return 0;
        }

        SEARCH_OPTIONS
            .into_iter()
            .map(|o| {
                o.into_iter()
                    .map(|i| index + i)
                    .map(|i| self.get(i?))
                    .zip(XMAS.into_iter().skip(1))
                    .all(|(l1, l2)| l1.is_some_and(|l| l == l2))
            })
            .map(u64::from)
            .sum()
    }
}
