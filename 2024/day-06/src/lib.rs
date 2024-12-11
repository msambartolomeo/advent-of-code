#![feature(btree_cursors)]

use std::collections::HashSet;

use maze::{Direction, Guard, Lookup};

pub mod maze;
pub mod parser;

pub type GuardPath = HashSet<((usize, usize), Direction)>;

#[must_use]
pub fn collapse_directions(visited: GuardPath) -> HashSet<(usize, usize)> {
    visited.into_iter().map(|(i, _)| i).collect()
}

#[must_use]
pub fn get_out(x_lookup: &Lookup, y_lookup: &Lookup, mut guard: Guard) -> Option<GuardPath> {
    let mut visited = GuardPath::default();

    loop {
        let direction = guard.direction();

        let obstacle = match direction {
            Direction::North => y_lookup.find_prev(guard.x(), guard.y()),
            Direction::South => y_lookup.find_next(guard.x(), guard.y()),
            Direction::East => x_lookup.find_next(guard.y(), guard.x()),
            Direction::West => x_lookup.find_prev(guard.y(), guard.x()),
        };

        let Some(obstacle) = obstacle else {
            let positions = match guard.direction() {
                Direction::North | Direction::West => guard.advance(0),
                Direction::South => guard.advance(x_lookup.len()),
                Direction::East => guard.advance(y_lookup.len()),
            };

            let added = add_visited(&mut visited, positions, direction);
            if !added {
                break None;
            }

            break Some(visited);
        };

        let positions = guard.advance(obstacle);
        let added = add_visited(&mut visited, positions, direction);
        if !added {
            break None;
        }
    }
}

pub fn add_visited(
    visited: &mut GuardPath,
    positions: impl Iterator<Item = (usize, usize)>,
    direction: Direction,
) -> bool {
    for p in positions {
        if visited.contains(&(p, direction)) {
            return false;
        }
        visited.insert((p, direction));
    }
    true
}
