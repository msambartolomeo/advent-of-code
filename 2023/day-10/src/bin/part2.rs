use anyhow::Result;
use day_10::Pipe;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u32> {
    let (pipes, start) = day_10::parse_pipes(input)?;

    let main_loop = pipes.pipe_loop_matrix(start);

    let mut result = 0;

    for (row, is_loop_row) in pipes.matrix.into_iter().zip(main_loop) {
        let mut loop_counter = 0;
        let mut last_bend: Option<Pipe> = None;

        for (pipe, is_loop) in row.into_iter().zip(is_loop_row) {
            if is_loop {
                match pipe {
                    Some(Pipe::Horizontal) | None => (),
                    Some(Pipe::Vertical) => loop_counter += 1,
                    Some(bend) => {
                        last_bend = last_bend.map_or(Some(bend), |last_bend| {
                            let continue_vertical = !bend
                                .openings()
                                .iter()
                                .any(|o| last_bend.openings().contains(o));

                            if continue_vertical {
                                loop_counter += 1;
                            }
                            None
                        });
                    }
                }
            } else if loop_counter % 2 == 1 {
                result += 1;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        4
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        8
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        10
    )]
    fn test_example(#[case] input: &str, #[case] expected: u32) {
        let result = process(input).unwrap();

        assert_eq!(expected, result);
    }
}
