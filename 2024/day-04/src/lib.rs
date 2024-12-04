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
pub struct I(pub isize, pub isize);
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
    pub fn search<R>(&self, index: U, pattern: &[Letter], options: &[R]) -> u64
    where
        R: AsRef<[I]>,
    {
        if self.get(index) != Some(pattern[0]) {
            return 0;
        }

        options
            .iter()
            .map(|o| {
                o.as_ref()
                    .iter()
                    .copied()
                    .map(|i| index + i)
                    .map(|i| self.get(i?))
                    .zip(pattern.iter().copied().skip(1))
                    .all(|(l1, l2)| l1.is_some_and(|l| l == l2))
            })
            .map(u64::from)
            .sum()
    }
}
