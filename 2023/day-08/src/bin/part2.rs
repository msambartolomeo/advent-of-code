use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let (directions, network) = day_08::parser::parse_maps(input)?;

    let result = network
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|n| day_08::get_steps_to_end(&directions, &network, n, |s| s.ends_with('Z')))
        .reduce(num::integer::lcm)
        .context("There should be at least 1 starting node")?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = process(input).unwrap();

        assert_eq!(6, result);
    }
}
