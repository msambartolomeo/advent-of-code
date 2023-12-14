use std::collections::BTreeMap;

use itertools::Itertools;

#[derive(Debug)]
pub struct Galaxy {
    pub x: usize,
    pub y: usize,
}

impl Galaxy {
    pub fn x_mut(&mut self) -> &mut usize {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut usize {
        &mut self.y
    }
}

pub fn parse_cosmos(input: &str) -> impl Iterator<Item = Galaxy> + '_ {
    input.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().filter_map(move |(x, c)| match c {
            '#' => Some(Galaxy { x, y }),
            _ => None,
        })
    })
}

pub fn expand_galaxy(galaxies: &mut [Galaxy], expansion_size: usize) {
    expand_galaxy_internal(galaxies, Galaxy::x_mut, expansion_size);
    expand_galaxy_internal(galaxies, Galaxy::y_mut, expansion_size);
}

fn expand_galaxy_internal<F>(galaxies: &mut [Galaxy], direction: F, expansion_size: usize)
where
    F: Fn(&mut Galaxy) -> &mut usize,
{
    let mut galaxies_by_line: BTreeMap<usize, Vec<&mut Galaxy>> =
        galaxies.iter_mut().fold(BTreeMap::new(), |mut map, g| {
            map.entry(*direction(g)).or_default().push(g);
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
            None => expanded += expansion_size - 1,
        }
    }
}

#[must_use]
pub fn shortest_paths(galaxies: &[Galaxy]) -> usize {
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| g1.x.abs_diff(g2.x) + g1.y.abs_diff(g2.y))
        .sum()
}
