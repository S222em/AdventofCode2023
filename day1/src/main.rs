use std::fs;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lines: Vec<&str> = file.split("\n").map(|line| line.trim()).collect();

    let mut sum: u64 = 0;

    for line in lines {
        let parsed = parse(line);

        let first = parsed.chars().find(|char| char.is_numeric()).unwrap_or('0');
        let last = parsed
            .chars()
            .rfind(|char| char.is_numeric())
            .unwrap_or('0');

        let mut result = String::from(first);
        result.push(last);

        sum += result.parse().unwrap_or(0);
    }

    println!("{}", sum);
}

fn parse(str: &str) -> String {
    let mut matches: Vec<_> = vec![];

    let mut parsed = str.to_string();

    for number in NUMBERS {
        let mut matched: Vec<_> = str.match_indices(number).collect();

        matches.append(&mut matched);
    }

    matches.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    let mut next: usize = 0;
    let mut replaced_count: usize = 0;

    let first = matches.first();
    let last = matches.last();

    if let Some((i, value)) = first {
        let range = *i..(i + value.len());

        parsed.replace_range(range, replace_with(value));

        next = i + value.len();
        replaced_count += value.len() - 1;
    }

    if let Some((i, value)) = last {
        if i + value.len() <= next {
            return parsed;
        }

        let mut range = (i - replaced_count)..(i + value.len() - replaced_count);

        if *i < next {
            let offset = next - i;

            range = (i + offset - replaced_count)..(i + value.len() - replaced_count);
        }

        parsed.replace_range(range, replace_with(value));
    }

    parsed
}

fn replace_with(str: &str) -> &str {
    match str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "\0",
    }
}
