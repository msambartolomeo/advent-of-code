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
        .filter_map(|mut v| {
            let mut next = v.split_first_mut();
            let mut keep = false;
            let empty = HashSet::new();

            while let Some((current, left)) = next {
                let set = rules.get(current).unwrap_or(&empty);

                let mut repeat = false;

                for (i, m) in left.to_vec().iter().enumerate() {
                    if !set.contains(m) {
                        keep = true;
                        repeat = true;
                        std::mem::swap(&mut *current, &mut left[i]);
                    }
                }

                if repeat {
                    next = Some((current, left));
                } else {
                    next = left.split_first_mut();
                }
            }

            keep.then_some(v[v.len() / 2])
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
        let expected: u64 = 123;

        let result = process(INPUT)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
