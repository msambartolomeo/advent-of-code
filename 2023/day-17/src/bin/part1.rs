use std::rc::Rc;

use anyhow::Result;
use day_17::{Actions, Crucible, SearchNode};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let city = day_17::parse::city(input)?;

    let first_node = SearchNode::new(Rc::new(city), Rc::new(NormalCrucible));

    Ok(day_17::get_heat_lost(first_node))
}

#[derive(Debug)]
struct NormalCrucible;

impl Crucible for NormalCrucible {
    fn actions(&self, moved_straigth: usize) -> std::slice::Iter<Actions> {
        if moved_straigth < 2 {
            [Actions::Straight, Actions::Right, Actions::Left].iter()
        } else {
            [Actions::Right, Actions::Left].iter()
        }
    }
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
