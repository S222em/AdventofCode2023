use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

const THREADS: usize = 6;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lines: Vec<_> = file.split("\n").collect();

    let mut matching: HashMap<usize, usize> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let line: Vec<_> = line
            .split(": ").last().unwrap()
            .split("| ").collect();

        let winning_numbers: Vec<_> = line[0].split_whitespace().collect();
        let numbers: Vec<_> = line[1].split_whitespace().collect();

        let mut matches: usize = 0;

        for winning_number in winning_numbers {
            if !numbers.contains(&winning_number) { continue; }

            matches += 1;
        }

        matching.insert(i + 1, matches);
    }

    let mut lookup: HashMap<usize, usize> = HashMap::new();

    let count: usize = process(1..=matching.len(), &matching, &mut lookup);

    println!("{}", count);
}

fn process(ids: RangeInclusive<usize>, card_value: &HashMap<usize, usize>, lookup: &mut HashMap<usize, usize>) -> usize {
    let mut total_count: usize = 0;

    for id in ids {
        if id > card_value.len() { continue; }

        let value = card_value.get(&id).unwrap();

        total_count += 1;

        if *value > 0 {
            if lookup.contains_key(&id) { total_count += lookup.get(&id).unwrap() } else {
                let count = process((id + 1)..=(id + value), card_value, lookup);
                lookup.insert(id, count);
                total_count += count;
            }
        };
    }

    total_count
}
