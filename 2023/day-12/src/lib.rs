use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, Copy)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            _ => bail!("{c} is an invalid spring"),
        }
    }
}

pub type Record = (Vec<Spring>, Vec<u32>);

pub fn unknown_spring_posibilities(record: &Record) -> u32 {
    unknown_spring_posibilities_rec(&record.0, &record.1, 0)
}

fn unknown_spring_posibilities_rec(
    springs: &[Spring],
    damaged_groups: &[u32],
    damaged_count: u32,
) -> u32 {
    match springs.split_first() {
        Some((Spring::Operational, springs)) => {
            if damaged_groups.first() == Some(&damaged_count) {
                unknown_spring_posibilities_rec(springs, &damaged_groups[1..], 0)
            } else if damaged_count == 0 {
                unknown_spring_posibilities_rec(springs, damaged_groups, 0)
            } else {
                0
            }
        }
        Some((Spring::Damaged, springs)) => match damaged_groups.first() {
            None => 0,
            Some(&group) if group == damaged_count => 0,
            _ => unknown_spring_posibilities_rec(springs, damaged_groups, damaged_count + 1),
        },
        Some((Spring::Unknown, _)) => {
            // Divide into damaged and operational posibilities
            let mut springs = springs.to_vec();

            springs[0] = Spring::Damaged;
            let damaged_posibilities =
                unknown_spring_posibilities_rec(&springs, damaged_groups, damaged_count);

            springs[0] = Spring::Operational;

            let operational_posibilities =
                unknown_spring_posibilities_rec(&springs, damaged_groups, damaged_count);

            damaged_posibilities + operational_posibilities
        }
        None => u32::from(
            damaged_groups.is_empty()
                || (damaged_groups.len() == 1 && damaged_groups[0] == damaged_count),
        ),
    }
}

pub fn parse_spring_records(input: &str) -> impl Iterator<Item = Result<Record>> + '_ {
    input.lines().map(parse_record)
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
