use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<i32> {
    let data = day_09::parse_oasis_report(input)?;

    let result = data
        .into_iter()
        .map(|v| {
            let mut finals = vec![];
            let mut next = v;
            loop {
                finals.push(*next.last().expect("At Least an element each iteration"));
                next = next
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();

                if next.iter().all(|n| *n == 0) {
                    break;
                }
            }
            finals.into_iter().sum::<i32>()
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

        let result = process(input).unwrap();

        assert_eq!(114, result);
    }
}
