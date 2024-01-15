use std::collections::HashSet;

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
    let mut last_frontier = HashSet::default();
    let mut switcher = Switcher::<u64>::default();
    switcher.init();

    for _ in 0..steps {
        let new_frontier = frontier
            .iter()
            .flat_map(|&pos| {
                pos.next().filter_map(|p| match garden.get(p)? {
                    Feature::Plot => {
                        (!frontier.contains(&p) && !last_frontier.contains(&p)).then_some(p)
                    }
                    Feature::Rock => None,
                })
            })
            .collect::<HashSet<_>>();

        *switcher.get_current() += frontier.len() as u64;

        last_frontier = frontier;
        frontier = new_frontier;

        switcher.switch();
    }

    let result = *switcher.get_current() + frontier.len() as u64;

    Ok(result)
}

#[derive(Debug)]
struct Switcher<T> {
    v1: T,
    v2: T,
    current: *mut T,
    other: *mut T,
}

impl<T: Default> Default for Switcher<T> {
    fn default() -> Self {
        Self {
            v1: Default::default(),
            v2: Default::default(),
            current: std::ptr::null_mut(),
            other: std::ptr::null_mut(),
        }
    }
}

impl<T> Switcher<T> {
    fn init(&mut self) {
        self.current = &mut self.v1;
        self.other = &mut self.v2;
    }

    fn is_init(&mut self) -> bool {
        (self.current == &mut self.v1 || self.current == &mut self.v2)
            && (self.other == &mut self.v1 || self.other == &mut self.v2)
    }

    #[must_use]
    fn get_current(&mut self) -> &mut T {
        assert!(self.is_init(), "Must initialize Switcher");

        unsafe { &mut *self.current }
    }

    fn switch(&mut self) -> &mut T {
        std::mem::swap(&mut self.current, &mut self.other);

        self.get_current()
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
