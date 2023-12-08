use anyhow::Result;
use day_07::parse_camel_cards;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let hands = parse_camel_cards(input)?;

    let result = hands
        .into_values()
        .enumerate()
        .map(|(rank, bid)| (rank as u64 + 1) * bid as u64)
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = process(input).unwrap();

        assert_eq!(6440, result);
    }
}
