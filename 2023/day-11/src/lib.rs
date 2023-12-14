#[derive(Debug)]
pub struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

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
