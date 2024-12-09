use anyhow::Result;
use day_09::BlockKind;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let fs = day_09::parser::parse(input)?;

    let mut fragments = fs.into_iter().flat_map(|ds| ds.fragments()).collect_vec();

    let mut empty_idx = 0;

    for block_idx in (0..fragments.len()).rev() {
        if matches!(fragments[block_idx], BlockKind::Empty) {
            continue;
        }

        if let Some(i) = fragments[empty_idx..block_idx]
            .iter()
            .position(|bk| matches!(bk, BlockKind::Empty))
        {
            empty_idx += i;
            fragments.swap(empty_idx, block_idx);
        } else {
            break;
        }
    }

    let result = fragments
        .into_iter()
        .enumerate()
        .map(|(i, ds)| i as u64 * ds.id())
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = 1928;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
