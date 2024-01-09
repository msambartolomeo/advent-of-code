use std::cmp::Ordering;
use std::collections::BTreeMap;

use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, Copy)]
pub enum Rating {
    ExtremelyCoolLooking,
    Musical,
    Aerodinamic,
    Shiny,
}

pub struct Part {
    extremely_cool_looking: u64,
    musical: u64,
    aerodinamic: u64,
    shiny: u64,
}

impl Part {
    fn rating(&self, rating_type: Rating) -> u64 {
        match rating_type {
            Rating::ExtremelyCoolLooking => self.extremely_cool_looking,
            Rating::Musical => self.musical,
            Rating::Aerodinamic => self.aerodinamic,
            Rating::Shiny => self.shiny,
        }
    }

    pub fn total_rating(&self) -> u64 {
        self.extremely_cool_looking + self.musical + self.aerodinamic + self.shiny
    }
}

pub enum Rule<'a> {
    Condition {
        rating: Rating,
        ordering: Ordering,
        value: u64,
        destination: &'a str,
    },
    Direct {
        destination: &'a str,
    },
}

impl<'a> Rule<'a> {
    pub fn apply(&self, part: &Part) -> RuleResult {
        match self {
            Rule::Condition {
                rating,
                ordering,
                value,
                destination,
            } => {
                let rating = part.rating(*rating);
                let condition = match ordering {
                    Ordering::Less => rating.lt(value),
                    Ordering::Equal => rating.eq(value),
                    Ordering::Greater => rating.gt(value),
                };

                if condition {
                    match *destination {
                        "A" => RuleResult::Accept,
                        "R" => RuleResult::Reject,
                        _ => RuleResult::SendTo(destination),
                    }
                } else {
                    RuleResult::Continue
                }
            }
            Rule::Direct { destination } => match *destination {
                "A" => RuleResult::Accept,
                "R" => RuleResult::Reject,
                _ => RuleResult::SendTo(destination),
            },
        }
    }
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
        .map(|rule| {
            if let Some((condition, destination)) = rule.split_once(':') {
                let mut chars = condition.chars();

                let rating = match chars.next() {
                    Some('x') => Rating::ExtremelyCoolLooking,
                    Some('m') => Rating::Musical,
                    Some('a') => Rating::Aerodinamic,
                    Some('s') => Rating::Shiny,
                    _ => bail!("Invalid rating"),
                };

                let ordering = match chars.next() {
                    Some('<') => Ordering::Less,
                    Some('=') => Ordering::Equal,
                    Some('>') => Ordering::Greater,
                    _ => bail!("Invalid ordering"),
                };

                let value = condition[2..].parse()?;

                Ok(Rule::Condition {
                    rating,
                    ordering,
                    value,
                    destination,
                })
            } else {
                Ok(Rule::Direct { destination: rule })
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
