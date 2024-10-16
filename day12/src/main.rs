use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let mut total_arrangements: usize = 0;

    for line in file.lines() {
        let (line, groups) = parse_line(line);
        let (line, groups) = duplicate_line(line, groups);

        total_arrangements += next(&line, &groups, &mut HashMap::new());
    }

    println!("{}", total_arrangements);
}

fn next(line: &str, groups: &[usize], cache: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    // Remove any . on the ends of the line. Operational characters on the ends don't matter.
    let trimmed_line = line.trim_start_matches('.');

    // If we have encountered this state before, return the result from the cache.
    if let Some(cached) = cache.get(&(trimmed_line.to_string(), Vec::from(groups))) {
        return *cached;
    }

    // If no groups are left and the line does not contain damaged characters, we have found a valid solution.
    // If the groups or line is empty we have an invalid solution
    if groups.is_empty() && !trimmed_line.contains('#') { return 1; } else if groups.is_empty() || trimmed_line.is_empty() { return 0; }

    // If the next group is larger than the current string we have an invalid combination
    if groups[0] > trimmed_line.len() { return 0; }

    let arrangements = match trimmed_line {
        // If the start of the line until the next groups length is all #, we found a group and can remove it from the line.
        trimmed_line if trimmed_line[..groups[0]].chars().all(|char| char == '#') => {
            let mut next_line = &trimmed_line[groups[0]..];
            // If the next char is #, the solution is not possible as there has to be a . between groups.
            if next_line.starts_with('#') { return 0; }
            // If the next char is a ? it has to be a . and can be removed
            if next_line.starts_with('?') { next_line = &next_line[1..]; }

            //let next_damaged_or_unknown = next_line.chars().position(|char| char == '#' || char == '?');
            //if next_damaged_or_unknown.is_none() && !next_line.is_empty() { return 0; }

            next(next_line, &groups[1..], cache)
        }
        trimmed_line if !trimmed_line[..groups[0]].contains('?') => { 0 }
        trimmed_line => {
            // If no ? are left at all invalid solution
            if !trimmed_line.chars().any(|char| char == '?') { return 0; }

            // Replace the next ? with . or #
            let next_line_operational = trimmed_line.replacen('?', ".", 1);
            let next_line_damaged = trimmed_line.replacen('?', "#", 1);

            next(&next_line_operational, groups, cache) + next(&next_line_damaged, groups, cache)
        }
    };

    // Add the result to the cache.
    cache.insert((trimmed_line.to_string(), Vec::from(groups)), arrangements);

    arrangements
}

fn duplicate_line(line: &str, groups: Vec<usize>) -> (String, Vec<usize>) {
    let duplicated_line = Vec::from([line]).repeat(5).join("?");
    let duplicated_groups = groups.repeat(5);

    (duplicated_line, duplicated_groups)
}

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let (line, groups_str) = line.split_once(" ").unwrap();
    let groups: Vec<_> = groups_str.split(",").map(|number| number.parse().unwrap()).collect();

    (line, groups)
}