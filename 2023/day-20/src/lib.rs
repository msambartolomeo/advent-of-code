use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;

use anyhow::{Context, Result};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Pulse {
    #[default]
    Low,
    High,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Low => f.write_str("low"),
            Pulse::High => f.write_str("high"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum State {
    On,
    #[default]
    Off,
}

#[derive(Debug)]
pub struct Order {
    pub pulse: Pulse,
    pub sender: String,
    pub reciever: String,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.sender, self.pulse, self.reciever)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Module<'a> {
    pub outputs: Vec<&'a str>,
    pub name: &'a str,
    pub state: ModuleType<'a>,
}

impl<'a> Module<'a> {
    pub fn recieve_and_send(
        &mut self,
        sender: &str,
        pulse: Pulse,
    ) -> Box<dyn Iterator<Item = Order> + '_> {
        let self_name = self.name;
        if let Some(pulse_to_send) = self.state.recieve_and_send(sender, pulse) {
            Box::new(self.outputs.iter().map(move |&output| Order {
                pulse: pulse_to_send,
                reciever: output.to_owned(),
                sender: self_name.to_string(),
            }))
        } else {
            Box::new(std::iter::empty())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ModuleType<'a> {
    FlipFlop(State),
    Conjunction(HashMap<&'a str, Pulse>),
    Broadcast,
}

impl<'a> ModuleType<'a> {
    fn recieve_and_send(&mut self, sender: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            ModuleType::FlipFlop(state) => match pulse {
                Pulse::Low => match state {
                    State::On => {
                        *state = State::Off;
                        Some(Pulse::Low)
                    }
                    State::Off => {
                        *state = State::On;
                        Some(Pulse::High)
                    }
                },
                Pulse::High => None,
            },
            ModuleType::Conjunction(map) => {
                *map.get_mut(sender)? = pulse;

                map.values()
                    .all(|&p| p == Pulse::High)
                    .then_some(Pulse::Low)
                    .or(Some(Pulse::High))
            }
            ModuleType::Broadcast => Some(pulse),
        }
    }
}

/// Parses module configuration creating the initializing the modules state
///
/// # Errors
/// If a module's format is not valid
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
        Some('%') => (&name[1..], ModuleType::FlipFlop(State::default())),
        Some('&') => (&name[1..], ModuleType::Conjunction(HashMap::default())),
        Some(_) if name == "broadcaster" => (name, ModuleType::Broadcast),
        _ => unreachable!("Invalid module type"),
    };

    let module = Module {
        outputs,
        name,
        state,
    };

    Ok((name, module))
}
