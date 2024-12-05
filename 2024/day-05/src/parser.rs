use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use itertools::Itertools;

type Rules = HashMap<u64, HashSet<u64>>;

pub fn parse(input: &str) -> Result<(Rules, Vec<Vec<u64>>)> {
    let (po, u) = input
        .trim()
        .split_once("\n\n")
        .context("Should Have two halfs")?;

    let page_ordering = parse_page_orderings(po)?;
    let updates = parse_updates(u)?;

    Ok((page_ordering, updates))
}

fn parse_page_orderings(input: &str) -> Result<Rules> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split_once('|')
                .context("Should have separator for ordering")
        })
        .map_ok(|(n1, n2)| Ok((n1.parse()?, n2.parse()?)))
        .flatten()
        .fold_ok(HashMap::new(), |mut map, (before, after)| {
            map.entry(before).or_default().insert(after);
            map
        })
}

fn parse_updates(input: &str) -> Result<Vec<Vec<u64>>> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().split(',').map(|n| Ok(n.parse()?)).collect())
        .collect()
}
