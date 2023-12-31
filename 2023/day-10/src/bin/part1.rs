use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<usize> {
    let (pipes, start) = day_10::parse_pipes(input)?;

    let result = pipes.pipe_loop(start).count() / 2;

    Ok(result)
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
    fn test_example(#[case] input: &str, #[case] expected: usize) {
        let result = process(input).unwrap();

        assert_eq!(expected, result);
    }
}
