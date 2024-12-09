use anyhow::{Context, Result};
use itertools::Itertools;

use crate::DiskItem;

pub fn parse(input: &str) -> Result<Vec<DiskItem>> {
    input
        .trim()
        .char_indices()
        .map(|(i, c)| {
            let len = c.to_digit(10).context("Invalid digit")?.into();

            Ok((0..len as usize).map(move |current| {
                if i % 2 == 0 {
                    let id = i as u64 / 2;
                    DiskItem::Block { id, current, len }
                } else {
                    DiskItem::EmptySpace { current, len }
                }
            }))
        })
        .flatten_ok()
        .collect()
}
