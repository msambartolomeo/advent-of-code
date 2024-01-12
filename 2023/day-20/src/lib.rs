use std::collections::{BTreeMap, HashMap};

use anyhow::{Context, Result};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Pulse {
    #[default]
    Low,
    High,
}

#[derive(Debug)]
pub struct Order<'a> {
    pulse: Pulse,
    reciever: &'a str,
}

#[derive(Debug)]
pub struct Module<'a> {
    outputs: Vec<&'a str>,
    state: ModuleType<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModuleType<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
    Broadcast,
}

impl<'a> Module<'a> {
    // pub fn send(&self) -> impl Iterator<Item = Order> {
    //     todo!()
    // }
}

pub fn parse_module_configuration(input: &str) -> Result<BTreeMap<&str, Module>> {
    let mut map = input
        .lines()
        .map(parse_module)
        .collect::<Result<BTreeMap<_, _>>>()?;

    let modules = map
        .iter()
        .map(|(&k, v)| (k, v.outputs.clone()))
        .collect::<Vec<_>>();

    for (name, outputs) in modules {
        for output in outputs {
            if let Some(Module {
                state: ModuleType::Conjunction(inputs),
                ..
            }) = map.get_mut(output)
            {
                inputs.insert(name, Pulse::default());
            }
        }
    }

    Ok(map)
}

fn parse_module(input: &str) -> Result<(&str, Module)> {
    let (name, outputs) = input
        .split_once(" -> ")
        .context("Module name and outputs must be separated by an arrow")?;

    let outputs = outputs.split(", ").collect::<Vec<_>>();

    let (name, state) = match name.chars().next() {
        Some('%') => (&name[1..], ModuleType::FlipFlop(false)),
        Some('&') => (&name[1..], ModuleType::Conjunction(HashMap::default())),
        Some(_) if name == "broadcaster" => (name, ModuleType::Broadcast),
        _ => unreachable!("Invalid module type"),
    };

    let module = Module { outputs, state };

    Ok((name, module))
}
