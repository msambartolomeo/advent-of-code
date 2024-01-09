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

#[derive(Debug, Clone, Copy)]
pub enum Ordering {
    Less,
    Greater,
    LessEq,
    GreaterEq,
}

impl Ordering {
    fn reverse(self) -> Self {
        match self {
            Ordering::Less => Ordering::GreaterEq,
            Ordering::Greater => Ordering::LessEq,
            Ordering::LessEq => Ordering::Greater,
            Ordering::GreaterEq => Ordering::Less,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    left: u64,
    right: u64,
}

impl Range {
    fn new(left: u64, right: u64) -> Self {
        Self { left, right }
    }

    fn decrease(&mut self, value: u64, ordering: Ordering) -> bool {
        if (self.left..=self.right).contains(&value) {
            match ordering {
                Ordering::Less => self.right = value - 1,
                Ordering::LessEq => self.right = value,
                Ordering::Greater => self.left = value + 1,
                Ordering::GreaterEq => self.left = value,
            }
            true
        } else {
            false
        }
    }

    fn join(self, other: Self) -> Self {
        Range {
            left: self.left.max(other.left),
            right: self.right.min(other.right),
        }
    }

    fn len(&self) -> u64 {
        self.right - self.left + 1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ValidParts {
    extremely_cool_looking: Range,
    musical: Range,
    aerodinamic: Range,
    shiny: Range,
}

impl ValidParts {
    pub fn new() -> Self {
        let range = Range::new(1, 4000);
        Self {
            extremely_cool_looking: range,
            musical: range,
            aerodinamic: range,
            shiny: range,
        }
    }

    #[must_use]
    fn decrease(&self, rating: Rating, value: u64, ordering: Ordering) -> Option<Self> {
        let mut valid_parts = *self;

        match rating {
            Rating::ExtremelyCoolLooking => {
                valid_parts.extremely_cool_looking.decrease(value, ordering)
            }
            Rating::Musical => valid_parts.musical.decrease(value, ordering),
            Rating::Aerodinamic => valid_parts.aerodinamic.decrease(value, ordering),
            Rating::Shiny => valid_parts.shiny.decrease(value, ordering),
        }
        .then_some(valid_parts)
    }

    pub fn join(self, other: Self) -> Self {
        Self {
            extremely_cool_looking: self
                .extremely_cool_looking
                .join(other.extremely_cool_looking),
            musical: self.musical.join(other.musical),
            aerodinamic: self.aerodinamic.join(other.aerodinamic),
            shiny: self.shiny.join(other.shiny),
        }
    }

    pub fn total_rating(&self) -> u64 {
        self.extremely_cool_looking.len()
            * self.musical.len()
            * self.aerodinamic.len()
            * self.shiny.len()
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
                    Ordering::Greater => rating.gt(value),
                    Ordering::LessEq => rating.le(value),
                    Ordering::GreaterEq => rating.ge(value),
                };

                RuleResult::from_destination(condition, destination)
            }
            Rule::Direct { destination } => RuleResult::from_destination(true, destination),
        }
    }

    pub fn apply_valid_parts(&self, valid_parts: &ValidParts) -> ValidPartsResult {
        match self {
            Rule::Condition {
                rating,
                ordering,
                value,
                destination,
            } => {
                let valid_parts_true = valid_parts.decrease(*rating, *value, *ordering);
                let result = RuleResult::from_destination(true, destination);
                let valid_parts_true = valid_parts_true.map(|p| (p, result));

                let valid_parts_false = valid_parts.decrease(*rating, *value, ordering.reverse());

                ValidPartsResult::Condition(valid_parts_true, valid_parts_false)
            }
            Rule::Direct { destination } => {
                ValidPartsResult::Direct(RuleResult::from_destination(true, destination))
            }
        }
    }
}

pub enum ValidPartsResult<'a> {
    Condition(Option<(ValidParts, RuleResult<'a>)>, Option<ValidParts>),
    Direct(RuleResult<'a>),
}

pub enum RuleResult<'a> {
    Continue,
    Accept,
    Reject,
    SendTo(&'a str),
}

impl<'a> RuleResult<'a> {
    fn from_destination(condition: bool, destination: &'a str) -> Self {
        if condition {
            match destination {
                "A" => RuleResult::Accept,
                "R" => RuleResult::Reject,
                _ => RuleResult::SendTo(destination),
            }
        } else {
            RuleResult::Continue
        }
    }
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
