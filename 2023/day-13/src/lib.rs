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
    /// # Panics
    /// if all rows dont have the same length
    #[must_use]
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

    #[must_use]
    pub fn rows(&self) -> MirrorAccessor {
        MirrorAccessor::Rows(self)
    }

    #[must_use]
    pub fn columns(&self) -> MirrorAccessor {
        MirrorAccessor::Columns(self)
    }
}

pub enum MirrorAccessor<'a> {
    Rows(&'a Mirror),
    Columns(&'a Mirror),
}

impl<'a> MirrorAccessor<'a> {
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            MirrorAccessor::Rows(m) => m.rows,
            MirrorAccessor::Columns(m) => m.columns,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn nth_line(&self, idx: usize) -> Box<dyn Iterator<Item = &Element> + '_> {
        match self {
            MirrorAccessor::Rows(m) => Box::new(m.matrix[idx].iter()),
            MirrorAccessor::Columns(m) => Box::new(m.matrix.iter().map(move |v| &v[idx])),
        }
    }

    #[must_use]
    pub fn lines(&self) -> Box<dyn Iterator<Item = Vec<&Element>> + '_> {
        match self {
            MirrorAccessor::Rows(m) => Box::new((0..m.rows).map(|i| self.nth_line(i).collect())),
            MirrorAccessor::Columns(m) => {
                Box::new((0..m.columns).map(|i| self.nth_line(i).collect()))
            }
        }
    }
}
/// Parses the environment into a vector of mirror patterns
///
/// # Errors
/// if input is invalid
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
