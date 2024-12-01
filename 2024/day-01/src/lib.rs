use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub mod parser;

pub type HistoricLocations = BinaryHeap<Reverse<u64>>;
