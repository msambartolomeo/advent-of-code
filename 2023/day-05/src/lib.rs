use anyhow::{bail, ensure, Context, Error, Ok, Result};

#[derive(Debug)]
pub struct AlmanacMap {
    pub source: AlmanacMapCategory,
    pub destination: AlmanacMapCategory,
    converters: Vec<MapRangeConverter>,
}

impl AlmanacMap {
    #[must_use]
    pub fn convert(&self, number: u64) -> u64 {
        for converter in &self.converters {
            if let Some(n) = converter.convert(number) {
                return n;
            }
        }
        number
    }
}

#[derive(Debug)]
pub struct MapRangeConverter {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl MapRangeConverter {
    #[must_use]
    fn convert(&self, number: u64) -> Option<u64> {
        if number >= self.source_start && number < self.source_start + self.length {
            Some(self.destination_start + number - self.source_start)
        } else {
            None
        }
    }
}

impl TryFrom<Vec<u64>> for MapRangeConverter {
    type Error = Error;

    fn try_from(value: Vec<u64>) -> Result<Self, Self::Error> {
        ensure!(value.len() >= 3, "Vector should have at least 3 elements");

        Ok(MapRangeConverter {
            destination_start: value[0],
            source_start: value[1],
            length: value[2],
        })
    }
}

#[derive(Debug)]
pub enum AlmanacMapCategory {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for AlmanacMapCategory {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "seed" => AlmanacMapCategory::Seed,
            "soil" => AlmanacMapCategory::Soil,
            "fertilizer" => AlmanacMapCategory::Fertilizer,
            "water" => AlmanacMapCategory::Water,
            "light" => AlmanacMapCategory::Light,
            "temperature" => AlmanacMapCategory::Temperature,
            "humidity" => AlmanacMapCategory::Humidity,
            "location" => AlmanacMapCategory::Location,
            _ => bail!("Invalid category"),
        })
    }
}

/// Parses seeds from input
///
/// # Example format
///
/// seeds: 79 14 55 13
///
/// # Errors
///
/// Returns an error if the input does not follow the format
pub fn parse_seeds(input: &str) -> Result<Vec<u64>> {
    let (seeds, numbers) = input
        .split_once(": ")
        .context("Seeds format must include :")?;

    if seeds != "seeds" {
        bail!("The input are not seeds");
    }

    numbers
        .split_whitespace()
        .map(|n| Ok(n.parse::<u64>()?))
        .collect()
}

/// Parses an `AlmanacMap` from input
///
/// # Example format
///
/// seed-to-soil map:
/// 50 98 2
/// 52 50 48
///
/// # Errors
///
/// Returns an error if the input does not follow the format
pub fn parse_map(input: &str) -> Result<AlmanacMap> {
    let mut lines = input.lines();

    let title = lines.next().context("Map must have a title")?;
    let (source, destination) = title
        .split_once("-to-")
        .context("Map title must follow format 'x-to-y'")?;
    let (destination, map) = destination
        .split_once(' ')
        .context("Map title must follow format '... map:'")?;

    if map != "map:" {
        bail!("The input is not a map");
    }

    let source = AlmanacMapCategory::try_from(source)?;
    let destination = AlmanacMapCategory::try_from(destination)?;

    let converters = lines
        .map(|l| MapRangeConverter::try_from(parse_numbers(l)?))
        .collect::<Result<Vec<MapRangeConverter>>>()?;

    Ok(AlmanacMap {
        source,
        destination,
        converters,
    })
}

#[inline]
fn parse_numbers(input: &str) -> Result<Vec<u64>> {
    input.split(' ').map(|n| Ok(n.parse::<u64>()?)).collect()
}
