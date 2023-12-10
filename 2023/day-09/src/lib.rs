use std::ops::Not;

use anyhow::Result;
use itertools::Itertools;

pub fn parse_oasis_report(input: &str) -> impl Iterator<Item = Result<Vec<i32>>> + '_ {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| Ok(n.parse()?)).collect())
}

pub fn differences(vec: Vec<i32>) -> impl Iterator<Item = Vec<i32>> {
    std::iter::successors(Some(vec), |vec| {
        vec.iter().all(|n| *n == 0).not().then_some(
            vec.iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<i32>>(),
        )
    })
}
