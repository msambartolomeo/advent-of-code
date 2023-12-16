use anyhow::{bail, Result};

#[derive(Debug, Default)]
pub enum Rock {
    Rounded,
    Cube,
    #[default]
    Empty,
}

impl TryFrom<char> for Rock {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            'O' => Ok(Rock::Rounded),
            '#' => Ok(Rock::Cube),
            '.' => Ok(Rock::Empty),
            _ => bail!("Invalid rock"),
        }
    }
}

pub fn parse_platform(input: &str) -> Result<Vec<Vec<Rock>>> {
    let mut columns: Vec<Vec<Rock>> = Vec::new();

    for l in input.lines() {
        for (i, c) in l.chars().enumerate() {
            let column = match columns.get_mut(i) {
                Some(v) => v,
                None => {
                    columns.push(Vec::new());
                    columns.get_mut(i).expect("Now new column was added")
                }
            };

            let rock: Rock = c.try_into()?;

            column.push(rock)
        }
    }

    Ok(columns)
}
