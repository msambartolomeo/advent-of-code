use anyhow::{bail, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Element {
    Ash,
    Rock,
}

impl TryFrom<char> for Element {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Element::Ash),
            '#' => Ok(Element::Rock),

            _ => bail!("Invalid character for environment element"),
        }
    }
}

#[derive(Debug)]
pub struct Mirror {
    matrix: Vec<Vec<Element>>,
    pub rows: usize,
    pub columns: usize,
}

impl Mirror {
    pub fn new(matrix: Vec<Vec<Element>>) -> Self {
        let rows = matrix.len();
        let columns = matrix[0].len();

        matrix.iter().for_each(|v| assert_eq!(columns, v.len()));

        Self {
            matrix,
            rows,
            columns,
        }
    }

    pub fn nth_row(&self, idx: usize) -> &Vec<Element> {
        &self.matrix[idx]
    }

    pub fn nth_column<'a>(&'a self, idx: usize) -> Vec<Element> {
        self.matrix.iter().map(move |v| v[idx]).collect()
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<Element>> + '_ {
        (0..self.rows).into_iter().map(|i| self.nth_row(i))
    }

    pub fn columns(&self) -> impl Iterator<Item = Vec<Element>> + '_ {
        (0..self.columns).into_iter().map(|i| self.nth_column(i))
    }
}

pub fn parse_environment(input: &str) -> Result<Vec<Mirror>> {
    input.split("\n\n").map(parse_mirror).collect()
}

fn parse_mirror(input: &str) -> Result<Mirror> {
    let matrix = input
        .lines()
        .map(|l| {
            l.chars()
                .map(TryInto::<Element>::try_into)
                .collect::<Result<Vec<Element>>>()
        })
        .collect::<Result<Vec<Vec<Element>>>>()?;

    Ok(Mirror::new(matrix))
}
