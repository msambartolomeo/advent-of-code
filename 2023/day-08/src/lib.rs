use std::collections::HashMap;

use anyhow::{format_err, Result};
use winnow::{
    ascii::{alphanumeric1, line_ending, multispace1},
    combinator::{
        alt, delimited, dispatch, eof, fail, repeat, separated_pair, success, terminated,
    },
    token::any,
    PResult, Parser,
};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub fn parse_maps(input: &str) -> Result<(Vec<Direction>, HashMap<&str, (&str, &str)>)> {
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

fn network<'a>(input: &mut &'a str) -> PResult<HashMap<&'a str, (&'a str, &'a str)>> {
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
