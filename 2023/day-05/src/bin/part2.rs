use std::ops::Range;

use day_05::AlmanacMap;

use anyhow::{Context, Ok, Result};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
    let seeds = day_05::parse_seeds(seeds)?;

    let maps = sections
        .map(day_05::parse_map)
        .collect::<Result<Vec<AlmanacMap>>>()?;

    let ranges = seeds
        .chunks_exact(2)
        .map(|range| range[0]..range[0] + range[1])
        .collect::<Vec<Range<u64>>>();

    // NOTE: Redundant clone allowed for more performance with rayon
    #[allow(clippy::redundant_clone)]
    let result = ranges
        .into_par_iter()
        .flat_map(|range| range.clone())
        .map(|seed| maps.iter().fold(seed, |seed, map| map.convert(seed)))
        .min()
        .context("Must exist minimum location")?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(46, result);
    }
}
