use anyhow::Result;
use day_08::Direction;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let (directions, network) = day_08::parse_maps(input)?;

    let result = directions
        .iter()
        .cycle()
        .fold_while((0, "AAA"), |(count, node), direction| {
            if node == "ZZZ" {
                return Done((count, node));
            }

            let current_node = network.get(node).expect("Node should exist");

            let next = match direction {
                Direction::Left => current_node.0,
                Direction::Right => current_node.1,
            };

            Continue((count + 1, next))
        })
        .into_inner()
        .0;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let result = process(input).unwrap();

        assert_eq!(2, result);
    }

    #[test]
    fn test_cycle() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let result = process(input).unwrap();

        assert_eq!(6, result);
    }
}
