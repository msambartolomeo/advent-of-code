use std::cmp::Reverse;
use std::collections::BinaryHeap;

use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;

use crate::HistoricLocations;

pub fn parse(input: &str) -> Result<(HistoricLocations, HistoricLocations)> {
    input
        .lines()
        .map(|l| l.split_once("   ").ok_or_else(|| anyhow!("Invalid line")))
        .map_ok(|l| Ok((l.0.parse()?, l.1.parse()?)))
        .flatten()
        .fold_ok(
            (BinaryHeap::new(), BinaryHeap::new()),
            |(mut v1, mut v2), (id1, id2)| {
                v1.push(Reverse(id1));
                v2.push(Reverse(id2));

                (v1, v2)
            },
        )
}
