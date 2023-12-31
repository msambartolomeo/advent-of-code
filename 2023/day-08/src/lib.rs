use std::collections::HashMap;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

type Network<'a> = HashMap<&'a str, (&'a str, &'a str)>;

/// Gets the steps to the end of the map following directions
///
/// # Panics
/// Should not panic if the Network was properly created
pub fn get_steps_to_end<F>(
    directions: &[Direction],
    network: &Network,
    starting_node: &str,
    ending_condition: F,
) -> u64
where
    F: Fn(&str) -> bool,
{
    directions
        .iter()
        .cycle()
        .fold_while((0, starting_node), |(count, node), direction| {
            if ending_condition(node) {
                return Done((count, node));
            }

            let current_node = network.get(node).expect("Node should exist");

            let next = match direction {
                Direction::Left => current_node.0,
                Direction::Right => current_node.1,
            };

            Continue((count + 1, next))
        })
        .into_inner()
        .0
}

pub mod parser {
    use super::{Direction, Network};

    use anyhow::{format_err, Result};
    use winnow::{
        ascii::{alphanumeric1, line_ending, multispace1},
        combinator::{
            alt, delimited, dispatch, eof, fail, repeat, separated_pair, success, terminated,
        },
        token::any,
        PResult, Parser,
    };

    /// Parses all the maps into a Direction vector and a network
    ///
    /// # Errors
    /// Errors if the input is not valid
    pub fn parse_maps(input: &str) -> Result<(Vec<Direction>, Network)> {
        separated_pair(directions, multispace1, network)
            .parse(input)
            .map_err(|e| format_err!(e.to_string()))
    }

    fn directions(input: &mut &str) -> PResult<Vec<Direction>> {
        repeat(
            1..,
            dispatch!(any;
                'L' => success(Direction::Left),
                'R' => success(Direction::Right),
                _ => fail,
            ),
        )
        .parse_next(input)
    }

    fn network<'a>(input: &mut &'a str) -> PResult<Network<'a>> {
        repeat(
            1..,
            terminated(
                separated_pair(
                    alphanumeric1,
                    " = ",
                    delimited('(', separated_pair(alphanumeric1, ", ", alphanumeric1), ')'),
                ),
                alt((line_ending, eof)),
            ),
        )
        .parse_next(input)
    }
}
