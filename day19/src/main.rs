use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::str::Lines;

// x, m, a, s
type Part = [Range<usize>; 4];
type Workflow<'a> = Vec<&'a str>;
type Workflows<'a> = HashMap<&'a str, Workflow<'a>>;

const START_WORKFLOW: &str = "in";
const ACCEPTED_WORKFLOW: &str = "A";
const REJECTED_WORKFLOW: &str = "R";
const RANGE_START: usize = 1;
const RANGE_END: usize = 4001;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();
    let (workflows, _) = file.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflows.lines());
    let mut accepted_parts: Vec<Part> = Vec::new();

    next(START_WORKFLOW, [RANGE_START..RANGE_END, RANGE_START..RANGE_END, RANGE_START..RANGE_END, RANGE_START..RANGE_END], &workflows, &mut accepted_parts);

    let mut total_combinations_count = 0;

    for accepted_part in accepted_parts {
        total_combinations_count += get_combinations_count(&accepted_part);
    }

    println!("{}", total_combinations_count);
}

fn get_combinations_count(part: &Part) -> usize {
    part.iter().fold(1, |acc, range| acc * range.len())
}

fn next(workflow: &str, mut part: Part, workflows: &Workflows, accepted_parts: &mut Vec<Part>) {
    if workflow == ACCEPTED_WORKFLOW { return accepted_parts.push(part); }
    if workflow == REJECTED_WORKFLOW { return; }

    for &rule in workflows.get(workflow).unwrap() {
        let result = apply_rule(rule, part.clone());
        if result.is_none() { continue; }
        let result = result.unwrap();

        next(result.0, result.1, workflows, accepted_parts);

        if let Some(rejected_part) = result.2 { part = rejected_part; }
    }
}

fn apply_rule(rule: &str, part: Part) -> Option<(&str, Part, Option<Part>)> {
    let index = rule.chars().position(|char| char == ':');
    if index.is_none() { return Some((rule, part, None)); }

    let category = &rule[0..1];
    let operand = &rule[1..2];
    let number: usize = rule[2..index?].parse().unwrap();
    let next_workflow = &rule[(index? + 1)..];

    let part_index = match category {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        _ => 3
    };

    let rule_accepted_range = match operand {
        ">" => (number + 1)..RANGE_END,
        _ => RANGE_START..number
    };

    let rule_rejected_range = match operand {
        ">" => RANGE_START..(number + 1),
        _ => number..RANGE_END
    };

    let accepted_range = fit(&part[part_index], &rule_accepted_range);
    let rejected_range = fit(&part[part_index], &rule_rejected_range);

    let mut accepted_part = part.clone();
    accepted_part[part_index] = accepted_range?;

    let rejected_part = if let Some(rejected_range) = rejected_range {
        let mut rejected_part = part.clone();
        rejected_part[part_index] = rejected_range;
        Some(rejected_part)
    } else { None };

    Some((next_workflow, accepted_part, rejected_part))
}

fn fit(a: &Range<usize>, b: &Range<usize>) -> Option<Range<usize>> {
    let mut range = a.clone();

    if b.start > a.end || b.end < a.start { return None; }

    if a.start < b.start { range.start = b.start; }
    if a.end > b.end { range.end = b.end; }

    Some(range)
}

fn parse_workflows(lines: Lines) -> Workflows {
    let mut workflows: Workflows = HashMap::new();

    for line in lines {
        let (name, rules) = line.split_once("{").unwrap();

        workflows.insert(name, rules.strip_suffix("}").unwrap().split(",").collect());
    }

    workflows
}
