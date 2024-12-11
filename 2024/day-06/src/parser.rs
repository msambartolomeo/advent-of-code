use std::collections::BTreeSet;

use anyhow::{bail, Context, Result};

use crate::maze::{Guard, Lookup};

pub fn parse(input: &str) -> Result<(Guard, Lookup, Lookup)> {
    let input = input.trim();
    let mut len_iter = input.lines();
    let width = len_iter.next().context("1 line")?.trim().len();
    let height = len_iter.count() + 1;

    let mut y_lookup = vec![BTreeSet::default(); width];
    let mut x_lookup = vec![BTreeSet::default(); height];
    let mut start = None;

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.trim().char_indices() {
            match c {
                '^' => start = Some((x, y)),
                '#' => {
                    y_lookup[x].insert(y);
                    x_lookup[y].insert(x);
                }
                '.' => (),
                _ => bail!("Invalid input"),
            }
        }
    }

    Ok((
        Guard::new(start.context("Must find start")?),
        Lookup(x_lookup),
        Lookup(y_lookup),
    ))
}
