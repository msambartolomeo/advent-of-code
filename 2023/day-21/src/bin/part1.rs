use std::collections::HashSet;
use std::hash::Hash;

use anyhow::Result;

use day_21::Feature;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input, 64)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str, steps: u32) -> Result<u64> {
    let garden = day_21::parse_garden(input)?;

    let mut frontier = HashSet::from([garden.start()]);
    let mut switcher = Switcher::default();
    switcher.init();

    for _ in 0..steps {
        let new_frontier = frontier
            .iter()
            .flat_map(|&pos| {
                pos.next().filter_map(|p| match garden.get(p)? {
                    Feature::Plot => {
                        (!frontier.contains(&p) && !switcher.contains(&p)).then_some(p)
                    }
                    Feature::Rock => None,
                })
            })
            .collect::<HashSet<_>>();

        let current = switcher.get_current();

        current.extend(frontier.into_iter());

        switcher.switch();

        frontier = new_frontier;
    }

    let result = frontier.len() as u64 + switcher.get_current().len() as u64;

    Ok(result)
}

#[derive(Debug)]
struct Switcher<T: Hash + Eq> {
    v1: HashSet<T>,
    v2: HashSet<T>,
    current: *mut HashSet<T>,
}

impl<T: Hash + Eq> Default for Switcher<T> {
    fn default() -> Self {
        Self {
            v1: HashSet::default(),
            v2: HashSet::default(),
            current: std::ptr::null_mut(),
        }
    }
}

impl<T: Hash + Eq> Switcher<T> {
    fn init(&mut self) {
        self.current = &mut self.v1;
    }

    #[must_use]
    fn get_current(&mut self) -> &mut HashSet<T> {
        assert!(!self.current.is_null(), "Must initialize Switcher");

        unsafe { &mut *self.current }
    }

    fn switch(&mut self) -> &mut HashSet<T> {
        if self.current == &mut self.v1 {
            self.current = &mut self.v2;
        } else if self.current == &mut self.v2 {
            self.current = &mut self.v1;
        } else {
            panic!("Must initialize switcher")
        }

        self.get_current()
    }

    fn contains(&self, other: &T) -> bool {
        self.v1.contains(other) || self.v2.contains(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let expected = 16;
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        let result = process(input, 6)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
