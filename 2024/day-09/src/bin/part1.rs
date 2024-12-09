use anyhow::Result;
use day_09::DiskItem;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let mut fs = day_09::parser::parse(input)?;

    let mut j = fs.len();

    for i in 0..fs.len() {
        if fs[i].is_empty() {
            if let Some(last) = last_block(&fs, j) {
                j = last
            } else {
                break;
            }
            if j < i {
                break;
            }

            fs.swap(i, j);
        }
    }

    let result = fs
        .into_iter()
        .enumerate()
        .map(|(i, ds)| i as u64 * ds.id())
        .sum();

    Ok(result)
}

fn last_block(v: &[DiskItem], start_from: usize) -> Option<usize> {
    v.iter()
        .enumerate()
        .rev()
        .skip(v.len() - start_from)
        .find(|(_, ds)| ds.is_block())
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
