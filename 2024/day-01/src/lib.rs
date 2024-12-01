pub mod parser;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub type HistoricLocations = BinaryHeap<Reverse<u64>>;
