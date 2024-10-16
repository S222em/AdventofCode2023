use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let steps: Vec<_> = file.trim().split(",").collect();
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();

    for step in steps {
        let label_end_index = step.chars().position(|char| char == '=' || char == '-').unwrap();
        let label = &step[..label_end_index];

        let hashed_label = hash_label(label);
        let box_contents = boxes.entry(hashed_label).or_default();

        let operation = &step[label_end_index..(1 + label_end_index)];

        let lens_position = box_contents.iter().position(|(label_b, _)| label == *label_b);

        if operation == "-" {
            if let Some(lens_position) = lens_position { box_contents.remove(lens_position); }
            continue;
        }

        let focal_length: usize = step[(label_end_index + 1)..].parse().unwrap();

        if let Some(lens_position) = lens_position {
            box_contents[lens_position] = (label, focal_length);
            continue;
        }

        box_contents.push((label, focal_length));
    }

    let mut focusing_power: usize = 0;

    for (hashed_label, contents) in boxes {
        for (i, (_, focal_length)) in contents.iter().enumerate() {
            focusing_power += (hashed_label + 1) * (i + 1) * focal_length;
        }
    }

    println!("{}", focusing_power)
}

fn hash_label(step: &str) -> usize {
    let mut value: usize = 0;

    for char in step.chars() {
        value += char as usize;
        value *= 17;
        value %= 256
    }

    value
}
