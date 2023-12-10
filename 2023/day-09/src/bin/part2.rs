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
    let data = day_09::parse_oasis_report(input);

    let result = data.process_results(|it| {
        it.map(|v| {
            day_09::differences(v)
                .map(|v| *v.first().expect("At least an element each iteration"))
                .collect_vec()
                .into_iter()
                .rfold(0, |acc, n| n - acc)
        })
        .sum()
    })?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

        let result = process(input).unwrap();

        assert_eq!(2, result);
    }
}
