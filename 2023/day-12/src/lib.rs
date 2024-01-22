use std::collections::HashMap;

use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => bail!("{c} is an invalid spring"),
        }
    }
}

pub type Record = (Vec<Spring>, Vec<u32>);

#[must_use]
pub fn repeat_record((springs, damaged): Record, times: usize) -> Record {
    let damaged = std::iter::repeat(damaged).take(times).flatten().collect();
    let springs = std::iter::repeat(springs)
        .take(times)
        .collect::<Vec<_>>()
        .join(&Spring::Unknown);

    (springs, damaged)
}

#[must_use]
pub fn unknown_spring_posibilities(record: Record) -> u64 {
    let mut cache = HashMap::new();

    unknown_spring_posibilities_rec(Box::from(record.0), &record.1, 0, &mut cache)
}

type CacheData<'a> = (Box<[Spring]>, &'a [u32], u32);

#[must_use]
fn unknown_spring_posibilities_rec<'a>(
    springs: Box<[Spring]>,
    damaged_groups: &'a [u32],
    damaged_count: u32,
    cache: &mut HashMap<CacheData<'a>, u64>,
) -> u64 {
    let cache_data = (springs, damaged_groups, damaged_count);

    cache.get(&cache_data).copied().unwrap_or_else(|| {
        let (springs, damaged_groups, damaged_count) = cache_data;
        let result = match springs.split_first() {
            Some((Spring::Operational, springs)) => {
                let springs = Box::from(springs);
                if damaged_groups.first() == Some(&damaged_count) {
                    unknown_spring_posibilities_rec(springs, &damaged_groups[1..], 0, cache)
                } else if damaged_count == 0 {
                    unknown_spring_posibilities_rec(springs, damaged_groups, 0, cache)
                } else {
                    0
                }
            }
            Some((Spring::Damaged, springs)) => match damaged_groups.first() {
                None => 0,
                Some(&group) if group == damaged_count => 0,
                _ => unknown_spring_posibilities_rec(
                    Box::from(springs),
                    damaged_groups,
                    damaged_count + 1,
                    cache,
                ),
            },
            Some((Spring::Unknown, _)) => {
                // Divide into damaged and operational posibilities
                let mut springs_clone = springs.to_vec();

                springs_clone[0] = Spring::Damaged;
                let damaged_posibilities = unknown_spring_posibilities_rec(
                    Box::from(springs_clone),
                    damaged_groups,
                    damaged_count,
                    cache,
                );

                let mut springs_clone = springs.to_vec();
                springs_clone[0] = Spring::Operational;

                let operational_posibilities = unknown_spring_posibilities_rec(
                    Box::from(springs_clone),
                    damaged_groups,
                    damaged_count,
                    cache,
                );

                damaged_posibilities + operational_posibilities
            }
            None => u64::from(
                damaged_groups.is_empty()
                    || (damaged_groups.len() == 1 && damaged_groups[0] == damaged_count),
            ),
        };

        cache.insert((springs, damaged_groups, damaged_count), result);
        result
    })
}

/// Calls the parse function for each spring record
/// # Errors
/// Errors if any record is invalid
pub fn parse_spring_records(input: &str) -> Result<Vec<Record>> {
    input.lines().map(parse_record).collect()
}

/// Parses a spring record composed of a list of springs and then the broken springs groups
/// # Errors
/// Errors if the input is not a valid record
pub fn parse_record(input: &str) -> Result<Record> {
    let (springs, damaged_groups) = input
        .split_once(' ')
        .context("input must have space separating springs and damaged")?;

    let springs = springs
        .chars()
        .map(Spring::try_from)
        .collect::<Result<_>>()?;

    let damaged_groups = damaged_groups
        .split(',')
        .map(|c| Ok(c.parse()?))
        .collect::<Result<_>>()?;

    Ok((springs, damaged_groups))
}
