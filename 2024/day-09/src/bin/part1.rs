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

    let mut j = fragments.len();

    for i in 0..fragments.len() {
        if matches!(fragments[i], BlockKind::Empty) {
            if let Some(last) = last_block(&fragments, j) {
                j = last;
            } else {
                break;
            }
            if j < i {
                break;
            }

            fragments.swap(i, j);
        }
    }

    let result = fragments
        .into_iter()
        .enumerate()
        .map(|(i, ds)| i as u64 * ds.id())
        .sum();

    Ok(result)
}

fn last_block(v: &[BlockKind], start_from: usize) -> Option<usize> {
    v.iter()
        .enumerate()
        .rev()
        .skip(v.len() - start_from)
        .find(|(_, bk)| matches!(bk, BlockKind::File(_)))
        .map(|(i, _)| i)
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
