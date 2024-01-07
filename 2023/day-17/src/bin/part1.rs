use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;

use anyhow::{Context, Result};
use day_17::SearchNode;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let city = day_17::parse::city(input)?;

    let start = (0, 0).into();
    let end = (city.length - 1, city.height - 1).into();

    let mut node = SearchNode::new(Rc::new(city), start, end);

    let mut posibilities = BinaryHeap::new();
    let mut cache = HashSet::new();

    while !node.is_goal() {
        posibilities.extend(node.succesors());

        loop {
            cache.insert(node);
            node = posibilities.pop().context("A path always exists")?;
            if !cache.contains(&node) {
                break;
            }
        }
    }

    Ok(node.heat_lost())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let result = process(input)?;

        assert_eq!(102, result);

        Ok(())
    }
}
