use anyhow::Result;
use day_18::{DigInstruction, ShoeLacePick};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let dig_plan = day_18::parse_dig_plan(input);

    let result = dig_plan
        .process_results(|it| {
            it.fold(ShoeLacePick::default(), |mut shoelace_pick, instruction| {
                let DigInstruction {
                    direction, meters, ..
                } = instruction;

                let last_position = shoelace_pick.last_vertex();

                let next_position = last_position.get_next(direction, meters);

                shoelace_pick.add_vertex(next_position, meters);

                shoelace_pick
            })
        })?
        .finish();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let result = process(input)?;

        assert_eq!(62, result);

        Ok(())
    }
}
