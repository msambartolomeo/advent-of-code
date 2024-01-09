use std::collections::BTreeMap;

use anyhow::Result;
use day_19::{Rule, RuleResult, ValidParts, ValidPartsResult};

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = process(input)?;

    println!("{result}");

    Ok(())
}

#[inline]
fn process(input: &str) -> Result<u64> {
    let (workflows, _) = day_19::parse_workflows_and_parts(input)?;

    let valid_parts = ValidParts::new();
    let mut accepted_parts = Vec::new();

    check_valid_parts("in", &workflows, valid_parts, &mut accepted_parts);

    let result = accepted_parts.iter().map(ValidParts::total_rating).sum();

    Ok(result)
}

pub fn check_valid_parts(
    name: &str,
    workflows: &BTreeMap<&str, Vec<Rule>>,
    mut valid_parts: ValidParts,
    accepted_parts: &mut Vec<ValidParts>,
) {
    let workflow = workflows.get(name).expect("workflow to exist");

    for rule in workflow {
        match rule.apply_valid_parts(&valid_parts) {
            ValidPartsResult::Condition(parts_true, parts_false) => {
                if let Some((parts, result)) = parts_true {
                    handle_rule_result(result, parts, workflows, accepted_parts);
                }

                if let Some(parts) = parts_false {
                    valid_parts = parts;
                } else {
                    break;
                }
            }

            ValidPartsResult::Direct(result) => {
                handle_rule_result(result, valid_parts, workflows, accepted_parts);
            }
        }
    }
}

#[inline]
fn handle_rule_result(
    result: RuleResult,
    parts: ValidParts,
    workflows: &BTreeMap<&str, Vec<Rule>>,
    accepted_parts: &mut Vec<ValidParts>,
) {
    match result {
        RuleResult::Continue => unreachable!("In each case the continue is handled in another way"),
        RuleResult::Accept => {
            accepted_parts.push(parts);
        }
        RuleResult::Reject => (),
        RuleResult::SendTo(destination) => {
            check_valid_parts(destination, workflows, parts, accepted_parts)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        let result = process(input)?;

        assert_eq!(167409079868000, result);

        Ok(())
    }
}
