use std::ops::Index;

pub mod parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PairIndex(usize, usize);

impl From<(usize, usize)> for PairIndex {
    fn from(value: (usize, usize)) -> Self {
        PairIndex(value.0, value.1)
    }
}

impl PairIndex {
    pub fn up(&self) -> Option<Self> {
        Some((self.0, self.1.checked_sub(1)?).into())
    }

    pub fn down(&self) -> Option<Self> {
        Some((self.0, self.1.checked_add(1)?).into())
    }

    pub fn left(&self) -> Option<Self> {
        Some((self.0.checked_sub(1)?, self.1).into())
    }

    pub fn right(&self) -> Option<Self> {
        Some((self.0.checked_add(1)?, self.1).into())
    }

    pub fn neighbors(&self) -> [Option<Self>; 4] {
        [self.up(), self.down(), self.left(), self.right()]
    }
}

#[derive(Debug)]
pub struct Matrix<T> {
    elements: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn get(&self, index: PairIndex) -> Option<&T> {
        self.elements.get(index.1).map(|v| v.get(index.0)).flatten()
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(elements: Vec<Vec<T>>) -> Self {
        Matrix { elements }
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = <MatrixIter<'a, T> as Iterator>::Item;

    type IntoIter = MatrixIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIter {
            matrix: &self,
            current: (0, 0).into(),
        }
    }
}

impl<T> Index<PairIndex> for Matrix<T> {
    type Output = T;

    fn index(&self, index: PairIndex) -> &Self::Output {
        &self.elements[index.1][index.0]
    }
}

pub struct MatrixIter<'a, T> {
    matrix: &'a Matrix<T>,

    current: PairIndex,
}

impl<'a, T> Iterator for MatrixIter<'a, T> {
    type Item = (PairIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .matrix
            .elements
            .get(self.current.1)
            .map(|v| {
                v.get(self.current.0).or_else(|| {
                    self.current.0 = 0;
                    self.current.1 += 1;
                    self.matrix.get(self.current)
                })
            })
            .flatten()
            .map(|n| (self.current, n));

        self.current.0 += 1;

        next
    }
}
