use std::cmp::Ordering;
use std::collections::BTreeMap;

use anyhow::{bail, Context, Result};

pub struct Part {
    extremely_cool_looking: u64,
    musical: u64,
    aerodinamic: u64,
    shiny: u64,
}

impl Part {
    fn extremely_cool_looking_rating(&self) -> u64 {
        self.extremely_cool_looking
    }

    fn musical_rating(&self) -> u64 {
        self.musical
    }

    fn aerodinamic_rating(&self) -> u64 {
        self.aerodinamic
    }

    fn shiny_rating(&self) -> u64 {
        self.shiny
    }

    pub fn total_rating(&self) -> u64 {
        self.extremely_cool_looking + self.musical + self.aerodinamic + self.shiny
    }
}

pub type Rule<'a> = Box<dyn Fn(&Part) -> RuleResult<'a> + 'a>;

pub fn rule_builder(
    condition: Option<(fn(&Part) -> u64, Ordering, u64)>,
    destination: &str,
) -> Rule {
    Box::new(move |part: &Part| -> RuleResult {
        let condition = if let Some((rating_getter, ordering, value)) = &condition {
            match ordering {
                Ordering::Less => rating_getter(part).lt(value),
                Ordering::Equal => rating_getter(part).eq(value),
                Ordering::Greater => rating_getter(part).gt(value),
            }
        } else {
            true
        };

        if condition {
            match destination {
                "A" => RuleResult::Accept,
                "R" => RuleResult::Reject,
                _ => RuleResult::SendTo(destination),
            }
        } else {
            RuleResult::Continue
        }
    })
}

pub enum RuleResult<'a> {
    Continue,
    Accept,
    Reject,
    SendTo(&'a str),
}

pub type Workflow<'a> = Vec<Rule<'a>>;

pub fn parse_workflows_and_parts(input: &str) -> Result<(BTreeMap<&str, Workflow>, Vec<Part>)> {
    let (workflows, parts) = input
        .split_once("\n\n")
        .context("Input must contain both parts")?;

    let workflows = workflows
        .lines()
        .map(parse_workflow)
        .collect::<Result<_>>()?;

    let parts = parts.lines().map(parse_part).collect::<Result<_>>()?;

    Ok((workflows, parts))
}

fn parse_workflow(input: &str) -> Result<(&str, Workflow)> {
    let (name, rules) = input
        .get(..input.len() - 1)
        .context("Must remove last bracket")?
        .split_once('{')
        .context("Rules must begin with a bracket")?;

    let rules = rules
        .split(',')
        .map(|rule| -> Result<Rule> {
            if let Some((condition, destination)) = rule.split_once(':') {
                let mut chars = condition.chars();

                let rating_getter = match chars.next() {
                    Some('x') => Part::extremely_cool_looking_rating,
                    Some('m') => Part::musical_rating,
                    Some('a') => Part::aerodinamic_rating,
                    Some('s') => Part::shiny_rating,
                    _ => bail!("Invalid rating"),
                };

                let ordering = match chars.next() {
                    Some('<') => Ordering::Less,
                    Some('=') => Ordering::Equal,
                    Some('>') => Ordering::Greater,
                    _ => bail!("Invalid ordering"),
                };

                let value = condition[2..].parse()?;

                let rule = rule_builder(Some((rating_getter, ordering, value)), destination);

                Ok(rule)
            } else {
                Ok(rule_builder(None, rule))
            }
        })
        .collect::<Result<_>>()?;

    Ok((name, rules))
}

fn parse_part(input: &str) -> Result<Part> {
    let [extremely_cool_looking, musical, aerodinamic, shiny] = input
        .get(1..input.len() - 1)
        .context("Must remove brackets")?
        .split(',')
        .map(|n| {
            let (_, n) = n.split_once('=').context("Equals must exist")?;
            Ok(n.parse()?)
        })
        .collect::<Result<Vec<_>>>()?[..]
    else {
        bail!("Must be only 4 values")
    };

    Ok(Part {
        extremely_cool_looking,
        musical,
        aerodinamic,
        shiny,
    })
}
