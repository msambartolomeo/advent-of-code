use std::collections::{BTreeMap, VecDeque};

use anyhow::Result;

use day_20::{Module, Order, Pulse};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

const TOTAL_TIMES: u32 = 1_000;

#[inline]
fn process(input: &str) -> Result<u32> {
    let mut modules = day_20::parse_module_configuration(input)?;

    let starting_modules = modules.clone();

    let mut times = 0;

    let mut pulses = Vec::new();
    loop {
        pulses.extend(push_button(&mut modules));

        times += 1;

        if starting_modules == modules || times == TOTAL_TIMES {
            break;
        }
    }

    let (high_pulses, low_pulses) =
        pulses
            .into_iter()
            .fold((0, 0), |(high_pulses, low_pulses), pulse| match pulse {
                Pulse::Low => (high_pulses, low_pulses + 1),
                Pulse::High => (high_pulses + 1, low_pulses),
            });

    let ratio = TOTAL_TIMES / times;
    let (high_pulses, low_pulses) = (high_pulses * ratio, low_pulses * ratio);
    let result = high_pulses * low_pulses;

    Ok(result)
}

fn push_button(modules: &mut BTreeMap<&str, Module>) -> Vec<Pulse> {
    let button_order = Order {
        pulse: Pulse::default(),
        sender: "button".to_owned(),
        reciever: "broadcaster".to_owned(),
    };

    let mut orders = VecDeque::from([button_order]);
    let mut pulses = Vec::new();

    while let Some(order) = orders.pop_front() {
        pulses.push(order.pulse);

        let Some(reciever) = modules.get_mut(order.reciever.as_str()) else {
            continue;
        };

        let new_orders = reciever.recieve_and_send(order.sender, order.pulse);

        orders.extend(new_orders);
    }
    pulses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invariable() -> Result<()> {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        let result = process(input)?;

        assert_eq!(32000000, result);

        Ok(())
    }

    #[test]
    fn test_variable() -> Result<()> {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        let result = process(input)?;

        assert_eq!(11687500, result);

        Ok(())
    }
}
