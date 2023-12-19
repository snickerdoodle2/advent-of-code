use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, anychar, newline},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

#[derive(Debug)]
enum When {
    LT(u32),
    GT(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Where<'a> {
    Reject,
    Accept,
    Next(&'a str),
}

#[derive(Debug)]
struct Rule<'a> {
    what: char,
    when: When,
    r#where: Where<'a>,
}

impl<'a> Rule<'a> {
    fn check(&self, part: &Part) -> Option<Where> {
        let v = match self.what {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => unreachable!(),
        };

        let valid = match self.when {
            When::LT(x) => v < x,
            When::GT(x) => v > x,
        };

        if valid {
            Some(self.r#where)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    r#final: Where<'a>,
}

impl<'a> Workflow<'a> {
    fn next_workflow(&self, part: &Part) -> Where {
        for rule in &self.rules {
            if let Some(where_) = rule.check(part) {
                return where_;
            }
        }
        self.r#final
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (what, when, amount, where_)) = tuple((
        anychar,
        alt((tag("<"), tag(">"))),
        complete::u32,
        preceded(tag(":"), alpha1).map(|e| match e {
            "A" => Where::Accept,
            "R" => Where::Reject,
            next => Where::Next(next),
        }),
    ))(input)?;
    let when = match when {
        "<" => When::LT(amount),
        ">" => When::GT(amount),
        _ => unreachable!(),
    };
    Ok((
        input,
        Rule {
            what,
            when,
            r#where: where_,
        },
    ))
}

fn parse_workflow(input: &str) -> IResult<&str, (&str, Workflow)> {
    let (input, name) = alpha1(input)?;
    let (input, (rules, final_)) = delimited(
        tag("{"),
        tuple((
            separated_list1(tag(","), parse_rule),
            preceded(
                tag(","),
                alpha1.map(|e| match e {
                    "A" => Where::Accept,
                    "R" => Where::Reject,
                    next => Where::Next(next),
                }),
            ),
        )),
        tag("}"),
    )(input)?;

    Ok((
        input,
        (
            name,
            Workflow {
                rules,
                r#final: final_,
            },
        ),
    ))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, (x, m, a, s)) = delimited(
        tag("{"),
        tuple((
            preceded(tag("x="), complete::u32),
            preceded(tag(",m="), complete::u32),
            preceded(tag(",a="), complete::u32),
            preceded(tag(",s="), complete::u32),
        )),
        tag("}"),
    )(input)?;

    Ok((input, Part { x, m, a, s }))
}

pub fn process_1(input: &str) -> String {
    let mut lines = input.split("\n\n");
    let workflows = separated_list1(newline, parse_workflow)(lines.next().unwrap())
        .unwrap()
        .1
        .into_iter()
        .collect::<HashMap<_, _>>();
    let (_, parts) = separated_list1(newline, parse_part)(lines.next().unwrap()).unwrap();

    parts
        .into_iter()
        .filter_map(|part| {
            let mut workflow_type = Where::Next("in");

            loop {
                if workflow_type == Where::Reject {
                    return None;
                }
                if workflow_type == Where::Accept {
                    return Some(part.x + part.m + part.a + part.s);
                }

                let workflow_name = match workflow_type {
                    Where::Next(name) => name,
                    _ => unreachable!(),
                };
                //println!("{}: {}", part.x, workflow_name);

                let workflow = workflows.get(&workflow_name).unwrap();
                workflow_type = workflow.next_workflow(&part);
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_2(_input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
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
        let res = process_1(input);
        assert_eq!("19114", res);
    }

    #[test]
    #[ignore]
    fn test_process_2() {
        let input = "";
        let res = process_2(input);
        assert_eq!("", res);
    }
}
