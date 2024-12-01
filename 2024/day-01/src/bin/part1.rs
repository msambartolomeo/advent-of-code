#![feature(binary_heap_into_iter_sorted)]

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let (hl1, hl2) = day_01::parser::parse(input)?;

    let result = hl1
        .into_iter_sorted()
        .zip(hl2.into_iter_sorted())
        .fold(0, |n, (id1, id2)| n + id1.0.abs_diff(id2.0));

    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    const INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = 11;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
