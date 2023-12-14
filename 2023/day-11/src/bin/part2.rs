use day_11::Galaxy;

fn main() {
    let input = include_str!("../../input.txt");

    let result = process(input, 1_000_000);

    println!("{result}");
}

#[inline]
fn process(input: &str, galaxy_expansion: usize) -> usize {
    let mut galaxies = day_11::parse_cosmos(input).collect::<Vec<Galaxy>>();

    day_11::expand_galaxy(&mut galaxies, galaxy_expansion);

    day_11::shortest_paths(&galaxies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
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

        let result = process(input, 10);

        assert_eq!(1030, result);
    }

    #[test]
    fn test_100() {
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

        let result = process(input, 100);

        assert_eq!(8410, result);
    }
}
