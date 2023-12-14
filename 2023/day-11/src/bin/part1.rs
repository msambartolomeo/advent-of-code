use std::collections::BTreeMap;

use anyhow::Result;
use day_11::Galaxy;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<usize> {
    let mut galaxies = day_11::parse_cosmos(input).collect_vec();

    expand_galaxy(&mut galaxies, Galaxy::x_mut);

    expand_galaxy(&mut galaxies, Galaxy::y_mut);

    let result = galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| g1.x().abs_diff(g2.x()) + g1.y().abs_diff(g2.y()))
        .sum();

    Ok(result)
}

fn expand_galaxy<F>(galaxies: &mut [Galaxy], direction: F)
where
    F: Fn(&mut Galaxy) -> &mut usize,
{
    let mut galaxies_by_line = galaxies.iter_mut().fold(BTreeMap::new(), |mut map, g| {
        map.entry(*direction(g)).or_insert(Vec::new()).push(g);
        map
    });

    let limit = galaxies_by_line
        .keys()
        .max()
        .expect("One galaxy must exist");

    let mut expanded = 0;

    for i in 0..=*limit {
        match galaxies_by_line.get_mut(&i) {
            Some(gs) => gs.iter_mut().for_each(|g| *direction(g) += expanded),
            None => expanded += 1,
        }
    }
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

        let result = process(input).unwrap();

        assert_eq!(374, result);
    }
}
