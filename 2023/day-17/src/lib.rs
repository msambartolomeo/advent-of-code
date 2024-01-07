use std::{hash::Hash, rc::Rc};

pub mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Position {
    pub fn move_to(&self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::North => (self.x, self.y.checked_sub(1)?).into(),
            Direction::South => (self.x, self.y.checked_add(1)?).into(),
            Direction::East => (self.x.checked_add(1)?, self.y).into(),
            Direction::West => (self.x.checked_sub(1)?, self.y).into(),
        })
    }
}

#[derive(Debug)]
pub struct City {
    pub length: usize,
    pub height: usize,
    matrix: Vec<Vec<u8>>,
}

impl City {
    pub fn distance(&self, from: Position, to: Position) -> usize {
        assert!(self.contains(from) && self.contains(to));

        to.x - from.x + to.y - from.y
    }

    pub fn contains(&self, position: Position) -> bool {
        position.x < self.length && position.y < self.height
    }

    pub fn get(&self, position: Position) -> u8 {
        assert!(self.contains(position));

        self.matrix[position.y][position.x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub enum Actions {
    Straight,
    Left,
    Right,
}

#[derive(Debug)]
pub struct SearchNode {
    pub city: Rc<City>,
    pub position: Position,
    goal: Position,
    heading: Direction,
    straight_count: usize,
    heat_loss_count: u32,
}

impl SearchNode {
    pub fn new(city: Rc<City>, position: Position, goal: Position) -> Self {
        Self {
            city,
            position,
            goal,
            heading: Direction::East,
            straight_count: 0,
            heat_loss_count: 0,
        }
    }

    pub fn succesors(&self) -> impl Iterator<Item = SearchNode> + '_ {
        let actions = if self.straight_count < 2 {
            [Actions::Straight, Actions::Right, Actions::Left].iter()
        } else {
            [Actions::Right, Actions::Left].iter()
        };

        actions.filter_map(move |action| {
            let (new_position, direction) = self.next_position(action)?;

            Some(SearchNode {
                city: Rc::clone(&self.city),
                position: new_position,
                goal: self.goal,
                heading: direction,
                straight_count: if let Actions::Straight = action {
                    self.straight_count + 1
                } else {
                    0
                },
                heat_loss_count: self.heat_loss_count + u32::from(self.city.get(new_position)),
            })
        })
    }

    fn next_position(&self, action: &Actions) -> Option<(Position, Direction)> {
        let new_direction = match (action, self.heading) {
            (Actions::Left, Direction::North) => Direction::West,
            (Actions::Left, Direction::South) => Direction::East,
            (Actions::Left, Direction::East) => Direction::North,
            (Actions::Left, Direction::West) => Direction::South,
            (Actions::Right, Direction::North) => Direction::East,
            (Actions::Right, Direction::South) => Direction::West,
            (Actions::Right, Direction::East) => Direction::South,
            (Actions::Right, Direction::West) => Direction::North,
            (Actions::Straight, direction) => direction,
        };

        let position = self.position.move_to(new_direction)?;

        self.city
            .contains(position)
            .then_some((position, new_direction))
    }

    pub fn is_goal(&self) -> bool {
        self.position == self.goal
    }

    pub fn heat_lost(&self) -> u32 {
        self.heat_loss_count
    }

    fn heuristic(&self) -> u32 {
        self.city.distance(self.position, self.goal) as u32
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.heading == other.heading
            && self.straight_count == other.straight_count
    }
}

impl Eq for SearchNode {}

/// The ord trait holds the heuristic to use in the A* algorithm plus the accumulated cost
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.heuristic() + self.heat_lost())
            .cmp(&(other.heuristic() + other.heat_lost()))
            .then(self.heat_lost().cmp(&other.heat_lost()))
            .reverse()
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for SearchNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.heading.hash(state);
        self.straight_count.hash(state);
    }
}
