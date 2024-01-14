use std::collections::BTreeMap;

use anyhow::{Context, Result};

use day_20::{Module, ModuleType};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let modules = day_20::parse_module_configuration(input)?;

    let broadcaster = modules.get("broadcaster").context("goal must exist")?;

    let conjunctions = broadcaster
        .outputs
        .iter()
        .map(|name| find_conjunction(name, &modules))
        .collect::<Result<Vec<_>>>()?;

    let result = conjunctions
        .into_iter()
        .map(|(first, conjunction)| binary_counter(first, 0, conjunction, &modules))
        .reduce(num::integer::lcm)
        .context("Expect at least a cycle")?;

    Ok(result)
}

fn find_conjunction<'a>(
    name: &str,
    modules: &'a BTreeMap<&str, Module>,
) -> Result<(&'a Module<'a>, &'a &'a str)> {
    let first_module = modules
        .get(name)
        .context("Exists because its an output of another module")?;

    let conjunction_name = first_module
        .outputs
        .iter()
        .filter(|&name| {
            matches!(
                modules
                    .get(name)
                    .expect("Must exist or input is invalid")
                    .state,
                ModuleType::Conjunction(_)
            )
        })
        .exactly_one()
        .map_err(|_| anyhow::format_err!("There must be exactly one conjunction that is the"))?;

    Ok((first_module, conjunction_name))
}

fn binary_counter(
    current_module: &Module,
    position: u32,
    conjunction: &str,
    modules: &BTreeMap<&str, Module>,
) -> u64 {
    let cumulative = if let Some((next_module,)) = current_module
        .outputs
        .iter()
        .filter_map(|&name| (name != conjunction).then_some(modules.get(name)?))
        .collect_tuple()
    {
        binary_counter(next_module, position + 1, conjunction, modules)
    } else {
        0
    };

    if current_module.outputs.contains(&conjunction) {
        cumulative + u64::from(2u32.pow(position))
    } else {
        cumulative
    }
}
