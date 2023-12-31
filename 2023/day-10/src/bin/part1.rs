use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let (pipes, start) = day_10::parse_pipes(input)?;

    let pipe = pipes.get(start).context("Starting pipe must exist")?;
    let mut position = start;
    let mut coordinate = pipe.openings()[0];

    let mut count = 0;
    loop {
        (position, coordinate) = pipes
            .get_next(position, coordinate)
            .context("Must form a loop")?;

        count += 1;

        if position == start {
            break;
        }
    }

    Ok(count / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        4
    )]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        4
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        8
    )]
    #[case(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        8
    )]
    fn test_square(#[case] input: &str, #[case] expected: u32) {
        let result = process(input).unwrap();

        assert_eq!(expected, result);
    }
}
