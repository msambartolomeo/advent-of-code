use day_11::Galaxy;

fn main() {
    let input = include_str!("../../input.txt");

    let result = process(input);

    println!("{result}");
}

#[inline]
fn process(input: &str) -> usize {
    let mut galaxies = day_11::parse_cosmos(input).collect::<Vec<Galaxy>>();

    day_11::expand_galaxy(&mut galaxies, 2);

    day_11::shortest_paths(&galaxies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = process(input);

        assert_eq!(374, result);
    }
}
