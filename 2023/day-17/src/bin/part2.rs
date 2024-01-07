use std::rc::Rc;

use anyhow::Result;
use day_17::{Actions, Crucible};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let city = day_17::parse::city(input)?;

    Ok(day_17::get_heat_lost(city, Rc::new(UltraCrucible)))
}

#[derive(Debug)]
struct UltraCrucible;

impl Crucible for UltraCrucible {
    fn actions(&self, moved_straigth: usize) -> std::slice::Iter<Actions> {
        if moved_straigth < 4 {
            [Actions::Straight].iter()
        } else if moved_straigth < 10 {
            [Actions::Straight, Actions::Right, Actions::Left].iter()
        } else {
            [Actions::Right, Actions::Left].iter()
        }
    }

    fn can_stop(&self, moved_straight: usize) -> bool {
        moved_straight >= 4
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

        assert_eq!(94, result);

        Ok(())
    }

    #[test]
    fn test_unfortunate() -> Result<()> {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";

        let result = process(input)?;

        assert_eq!(71, result);

        Ok(())
    }
}
