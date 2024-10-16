use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lines: Vec<_> = file.split("\n").collect();

    let (instructions, lines) = lines.split_first().unwrap();

    let instructions: Vec<usize> = instructions.chars().map(|char| {
        if char == 'L' { 0 } else { 1 }
    }).collect();

    let hash = parse(lines);

    let start_nodes: Vec<&str> = hash.iter().filter_map(|(&node, _)| {
        if node.ends_with('A') { Some(node) } else { None }
    }).collect();

    let mut all_steps: Vec<usize> = Vec::new();

    for start_node in start_nodes {
        let mut current_node: &str = start_node;
        let mut steps: usize = 0;

        for &instruction in instructions.iter().cycle() {
            if current_node.ends_with('Z') { break; }

            current_node = hash.get(current_node).unwrap()[instruction];

            steps += 1;
        }

        all_steps.push(steps);
    }

    let steps = all_steps.iter().fold(1, |acc: usize, step: &usize| least_common_multiple(acc, *step));

    println!("It took {} steps", steps);
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a / greatest_common_divider(a, b) * b
}

fn greatest_common_divider(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    greatest_common_divider(b, a % b)
}

fn parse<'a>(lines: &'a [&str]) -> HashMap<&'a str, [&'a str; 2]> {
    let mut hash = HashMap::new();

    for &line in lines.iter().skip(1) {
        let node = &line[0..3];
        let node_left = &line[7..10];
        let node_right = &line[12..15];

        hash.insert(node, [node_left, node_right]);
    }

    hash
}
