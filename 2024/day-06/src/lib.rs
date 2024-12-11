#![feature(btree_cursors)]

use std::collections::BTreeSet;
use std::ops::Bound;

pub mod parser;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct Guard {
    position: (usize, usize),
    direction: Direction,
}

#[derive(Debug)]
pub struct Lookup(Vec<BTreeSet<usize>>);

impl Lookup {
    #[must_use]
    pub fn find_next(&self, context_index: usize, current_index: usize) -> Option<usize> {
        self.0[context_index]
            .lower_bound(Bound::Excluded(&current_index))
            .next()
            .copied()
    }

    #[must_use]
    pub fn find_prev(&self, context_index: usize, current_index: usize) -> Option<usize> {
        self.0[context_index]
            .upper_bound(Bound::Excluded(&current_index))
            .prev()
            .copied()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Guard {
    const fn new(position: (usize, usize)) -> Self {
        Self {
            position,
            direction: Direction::North,
        }
    }

    #[must_use]
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    #[must_use]
    pub const fn x(&self) -> usize {
        self.position.0
    }

    #[must_use]
    pub const fn y(&self) -> usize {
        self.position.1
    }

    pub fn advance(&mut self, obstacle: usize) -> impl Iterator<Item = (usize, usize)> {
        let old_direction = self.direction;
        self.direction = old_direction.next();
        let position = self.position;

        let range = match old_direction {
            Direction::North => {
                let p = std::mem::replace(&mut self.position.1, obstacle + 1);
                obstacle + 1..p
            }
            Direction::South => {
                let p = std::mem::replace(&mut self.position.1, obstacle - 1);
                p..obstacle
            }

            Direction::East => {
                let p = std::mem::replace(&mut self.position.0, obstacle - 1);
                p..obstacle
            }
            Direction::West => {
                let p = std::mem::replace(&mut self.position.0, obstacle + 1);
                obstacle + 1..p
            }
        };

        range.map(move |n| match old_direction {
            Direction::North | Direction::South => (position.0, n),
            Direction::East | Direction::West => (n, position.1),
        })
    }
}

impl Direction {
    const fn next(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[test]
fn lookup() {
    let mut v = vec![BTreeSet::new()];

    v[0].insert(1);
    v[0].insert(3);
    v[0].insert(5);

    let l = Lookup(v);

    assert_eq!(l.find_next(0, 2), Some(3));
    assert_eq!(l.find_prev(0, 4), Some(3));
    assert_eq!(l.find_next(0, 5), None);
    assert_eq!(l.find_prev(0, 1), None);
}
