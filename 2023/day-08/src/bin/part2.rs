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
fn process(input: &str) -> Result<u64> {
    let (directions, network) = day_08::parse_maps(input)?;

    let starting_nodes = network
        .keys()
        .filter(|s| s.ends_with('A'))
        .collect::<Vec<_>>();

    let result = starting_nodes
        .into_iter()
        .map(|n| {
            directions
                .iter()
                .cycle()
                .fold_while((0, *n), |(count, node), direction| {
                    if node.ends_with('Z') {
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
                .0
        })
        .fold(1, num::integer::lcm);

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
