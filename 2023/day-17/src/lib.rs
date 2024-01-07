#[cfg(feature = "print_grid")]
use std::collections::HashMap;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

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
    #[must_use]
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
    /// # Panics
    /// If position is out of the city
    #[must_use]
    pub fn distance(&self, from: Position, to: Position) -> usize {
        assert!(self.contains(from) && self.contains(to));

        to.x - from.x + to.y - from.y
    }

    #[must_use]
    pub fn contains(&self, position: Position) -> bool {
        position.x < self.length && position.y < self.height
    }

    /// # Panics
    /// If position is out of the city
    #[must_use]
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node {
    position: Position,
    heading: Direction,
    moved_straight: usize,
}

#[derive(Debug)]
pub struct SearchNode {
    #[cfg(feature = "print_grid")]
    path: HashMap<Position, ()>,
    city: Rc<City>,
    goal: Position,
    crucible: Rc<dyn Crucible>,
    heat_loss_count: u32,
    node: Node,
}

impl SearchNode {
    pub fn new(city: Rc<City>, crucible: Rc<dyn Crucible>, direction: Direction) -> Self {
        let goal = (city.length - 1, city.height - 1).into();
        let node = Node {
            position: (0, 0).into(),
            heading: direction,
            moved_straight: 1,
        };

        Self {
            #[cfg(feature = "print_grid")]
            path: HashMap::from([((0, 0).into(), ())]),
            city,
            goal,
            heat_loss_count: 0,
            crucible,
            node,
        }
    }

    pub fn succesors(&self) -> impl Iterator<Item = SearchNode> + '_ {
        self.crucible
            .actions(self.node.moved_straight)
            .filter_map(|&action| {
                let (position, heading) = self.next_position(action)?;
                let moved_straight = if let Actions::Straight = action {
                    self.node.moved_straight + 1
                } else {
                    1
                };
                let heat_loss_count = self.heat_loss_count + u32::from(self.city.get(position));
                let node = Node {
                    position,
                    heading,
                    moved_straight,
                };

                Some(SearchNode {
                    #[cfg(feature = "print_grid")]
                    path: {
                        let mut path = self.path.clone();
                        path.insert(position, ());
                        path
                    },
                    city: Rc::clone(&self.city),
                    goal: self.goal,
                    heat_loss_count,
                    crucible: Rc::clone(&self.crucible),
                    node,
                })
            })
    }

    #[must_use]
    fn next_position(&self, action: Actions) -> Option<(Position, Direction)> {
        let new_direction = match (action, self.node.heading) {
            (Actions::Left, Direction::North) | (Actions::Right, Direction::South) => {
                Direction::West
            }
            (Actions::Left, Direction::South) | (Actions::Right, Direction::North) => {
                Direction::East
            }
            (Actions::Left, Direction::East) | (Actions::Right, Direction::West) => {
                Direction::North
            }
            (Actions::Left, Direction::West) | (Actions::Right, Direction::East) => {
                Direction::South
            }
            (Actions::Straight, direction) => direction,
        };

        let position = self.node.position.move_to(new_direction)?;

        self.city
            .contains(position)
            .then_some((position, new_direction))
    }

    #[must_use]
    pub fn is_goal(&self) -> bool {
        self.city.distance(self.node.position, self.goal) == 0
            && self.crucible.can_stop(self.node.moved_straight)
    }

    #[must_use]
    pub fn heat_lost(&self) -> u32 {
        self.heat_loss_count
    }

    #[must_use]
    fn heuristic(&self) -> u32 {
        u32::try_from(self.city.distance(self.node.position, self.goal)).unwrap()
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
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
        self.node.hash(state);
    }
}

/// # Panics
/// If there is no posible path to the end of the city
#[must_use]
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

    #[cfg(feature = "print_grid")]
    for y in 0..node.city.height {
        for x in 0..node.city.length {
            match node.path.get(&(x, y).into()) {
                Some(()) => {
                    print!("#");
                }
                None => print!("."),
            }
        }
        println!();
    }

    node.heat_lost()
}
