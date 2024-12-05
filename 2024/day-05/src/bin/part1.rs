use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let (rules, updates) = day_05::parser::parse(input)?;

    let result = updates
        .into_iter()
        .filter_map(|v| {
            let mut next = v.split_first();
            let empty = HashSet::new();

            while let Some((current, left)) = next {
                let set = rules.get(current).unwrap_or(&empty);

                for m in left {
                    if !set.contains(m) {
                        return None;
                    }
                }

                next = left.split_first();
            }

            Some(v[v.len() / 2])
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    const INPUT: &str = "
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    ";

    #[test]
    fn test_example() -> Result<()> {
        let expected: u64 = 143;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
