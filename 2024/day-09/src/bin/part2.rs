use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let mut fs = day_09::parser::parse(input)?;

    for block_idx in (0..fs.len()).rev() {
        let block = fs[block_idx];
        if block.is_empty() {
            continue;
        }

        if let Some(empty_idk) = fs[0..block_idx]
            .iter()
            .position(|bk| bk.is_empty() && bk.len >= block.len)
        {
            let empty = fs[empty_idk];
            let block = fs[block_idx];

            let (empty, part) = empty.partition(block.len);

            fs[empty_idk] = block;
            fs[block_idx] = empty;

            if let Some(empty) = part {
                fs.insert(empty_idk + 1, empty);
            }
        }
    }

    let result = fs
        .into_iter()
        .flat_map(|ds| ds.fragments())
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
        let expected: u64 = 2858;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
