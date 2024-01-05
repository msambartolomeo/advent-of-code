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
