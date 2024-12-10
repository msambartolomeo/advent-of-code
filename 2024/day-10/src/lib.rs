use std::collections::HashSet;

use matrix::{Matrix, PairIndex};

pub mod matrix;
pub mod parser;

#[must_use]
pub fn trails_dfs(topographic_map: &Matrix<u64>, start: PairIndex) -> u64 {
    trail_dfs_inner(topographic_map, start, None)
}

#[must_use]
pub fn trail_tails_dfs(topographic_map: &Matrix<u64>, start: PairIndex) -> u64 {
    trail_dfs_inner(topographic_map, start, Some(HashSet::new()))
}

fn trail_dfs_inner(
    topographic_map: &Matrix<u64>,
    start: PairIndex,
    mut trail_tails: Option<HashSet<PairIndex>>,
) -> u64 {
    let mut stack = vec![start];
    let mut trails = 0;

    while let Some(idx) = stack.pop() {
        let current = topographic_map[idx];

        if current == 9 {
            if let Some(s) = trail_tails.as_mut() {
                s.insert(idx);
            }
            trails += 1;
            continue;
        }

        idx.neighbors()
            .into_iter()
            .filter_map(|idx| {
                let idx = idx?;
                let n = *topographic_map.get(idx)?;

                (n == current + 1).then_some(idx)
            })
            .for_each(|i| stack.push(i));
    }

    trail_tails.map_or(trails, |tt| tt.len() as u64)
}
