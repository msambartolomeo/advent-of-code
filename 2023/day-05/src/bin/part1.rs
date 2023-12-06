use anyhow::{Context, Ok, Result};
use day_05::AlmanacMap;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let mut sections = input.split("\n\n");

    let seeds = sections.next().context("First section must be seeds")?;
    let mut seeds = day_05::parse_seeds(seeds)?;

    let almanacs = sections
        .map(|s| day_05::parse_map(s))
        .collect::<Result<Vec<AlmanacMap>>>()?;

    almanacs.into_iter().for_each(|map| {
        seeds.iter_mut().for_each(|s| *s = map.convert(*s));
    });

    let result = seeds.into_iter().min().context("Vector must have seeds")?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn test_example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = process(input).unwrap();

        assert_eq!(35, result);
    }

    #[rstest]
    #[case(
        "seed-to-soil map:\n50 98 2\n52 50 48",
        [79,14,55,13],
        [81,14,57,13],
    )]
    #[case(
        "soil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15",
        [81,14,57,13],
        [81,53,57,52],
    )]
    #[case(
        "fertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4",
        [81,53,57,52],
        [81,49,53,41],
    )]
    #[case(
        "water-to-light map:\n88 18 7\n18 25 70",
        [81,49,53,41],
        [74,42,46,34],
    )]
    #[case(
        "light-to-temperature map:\n45 77 23\n81 45 19\n68 64 13",
        [74,42,46,34],
        [78,42,82,34],
    )]
    #[case(
        "temperature-to-humidity map:\n0 69 1\n1 0 69",
        [78,42,82,34],
        [78,43,82,35],
    )]
    #[case(
        "humidity-to-location map:\n60 56 37\n56 93 4",
        [78,43,82,35],
        [82,43,86,35],
    )]
    fn map_test(#[case] map: &str, #[case] numbers: [u64; 4], #[case] expected: [u64; 4]) {
        let map = day_05::parse_map(map).unwrap();

        numbers
            .into_iter()
            .zip(expected)
            .for_each(|(n, e)| assert_eq!(e, map.convert(n)));
    }
}
