use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

pub mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

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

#[derive(Debug, Clone, Copy)]
pub enum Actions {
    Straight,
    Left,
    Right,
}

pub trait Crucible: Debug {
    fn actions(&self, moved_straigth: usize) -> std::slice::Iter<Actions>;
    fn can_stop(&self, moved_straight: usize) -> bool;
}

#[derive(Debug)]
pub struct SearchNode {
    city: Rc<City>,
    position: Position,
    goal: Position,
    heading: Direction,
    straight_count: usize,
    heat_loss_count: u32,
    crucible: Rc<dyn Crucible>,
}

impl SearchNode {
    pub fn new(city: Rc<City>, crucible: Rc<dyn Crucible>, direction: Direction) -> Self {
        let goal = (city.length - 1, city.height - 1).into();

        Self {
            city,
            position: (0, 0).into(),
            goal,
            heading: direction,
            straight_count: 1,
            heat_loss_count: 0,
            crucible,
        }
    }

    pub fn succesors(&self) -> impl Iterator<Item = SearchNode> + '_ {
        self.crucible
            .actions(self.straight_count)
            .filter_map(|action| {
                let (new_position, direction) = self.next_position(action)?;
                let straight = if let Actions::Straight = action {
                    self.straight_count + 1
                } else {
                    1
                };

                Some(SearchNode {
                    city: Rc::clone(&self.city),
                    position: new_position,
                    goal: self.goal,
                    heading: direction,
                    straight_count: straight,
                    heat_loss_count: self.heat_loss_count + u32::from(self.city.get(new_position)),
                    crucible: Rc::clone(&self.crucible),
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
        self.position == self.goal && self.crucible.can_stop(self.straight_count)
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
            .then(self.heuristic().cmp(&other.heuristic()))
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

pub fn get_heat_lost(city: City, crucible: Rc<dyn Crucible>) -> u32 {
    let city = Rc::new(city);
    let mut node = SearchNode::new(Rc::clone(&city), Rc::clone(&crucible), Direction::East);
    let mut posibilities = BinaryHeap::from([SearchNode::new(city, crucible, Direction::South)]);
    let mut cache = HashSet::new();

    while !node.is_goal() {
        posibilities.extend(node.succesors());

        loop {
            cache.insert(node);
            node = posibilities.pop().expect("A path always exists");
            if !cache.contains(&node) {
                break;
            }
        }
    }

    node.heat_lost()
}
