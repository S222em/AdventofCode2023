use std::fs;

#[derive(PartialEq)]
enum Mode {
    Horizontal,
    Vertical,
}

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let patterns: Vec<_> = file.split("\n\n").collect();

    let mut sum: usize = 0;

    for pattern in patterns {
        sum += analyze_pattern(pattern);
    }

    println!("{}", sum)
}

fn analyze_pattern(pattern: &str) -> usize {
    let pattern: Vec<Vec<char>> = pattern.lines().map(|str| str.chars().collect()).collect();

    let possible_reflection_line = 'reflection_line: {
        let horizontal_reflection_line = find_reflection(&pattern, Mode::Horizontal, None, None);
        if let Some(reflection_line) = horizontal_reflection_line { break 'reflection_line Some((reflection_line, Mode::Horizontal)); }

        let vertical_reflection_line = find_reflection(&pattern, Mode::Vertical, None, None);
        if let Some(reflection_line) = vertical_reflection_line { break 'reflection_line Some((reflection_line, Mode::Vertical)); }

        None
    };

    if possible_reflection_line.is_none() { return 0; }

    let (original_reflection_line, original_mode) = possible_reflection_line.unwrap();

    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            let reflection_line = find_reflection(&pattern, Mode::Horizontal, if original_mode == Mode::Horizontal { Some(original_reflection_line) } else { None }, Some((i, j)));

            if reflection_line.is_none() { continue; }

            return apply_weight(reflection_line.unwrap(), Mode::Horizontal);
        }
    }

    for i in 0..pattern[0].len() {
        for j in 0..pattern.len() {
            let reflection_line = find_reflection(&pattern, Mode::Vertical, if original_mode == Mode::Vertical { Some(original_reflection_line) } else { None }, Some((i, j)));

            if reflection_line.is_none() { continue; }

            return apply_weight(reflection_line.unwrap(), Mode::Vertical);
        }
    }

    apply_weight(original_reflection_line, original_mode)
}

fn apply_weight(i: usize, mode: Mode) -> usize {
    if mode == Mode::Horizontal { i * 100 } else { i }
}

fn find_reflection(pattern: &[Vec<char>], mode: Mode, not: Option<usize>, flip: Option<(usize, usize)>) -> Option<usize> {
    let mut range = match mode {
        Mode::Horizontal => 1..pattern.len(),
        Mode::Vertical => 1..pattern[0].len()
    };

    range.find(|&i| are_equal(i - 1, i, pattern, &mode, &flip) && verify_reflection(i, pattern, &mode, &flip) && i != not.unwrap_or(i - 1))
}

fn verify_reflection(reflection_line: usize, pattern: &[Vec<char>], mode: &Mode, flip: &Option<(usize, usize)>) -> bool {
    if reflection_line == 1 { return true; }

    let range_i = 0..(reflection_line - 1);
    let range_j = match *mode {
        Mode::Horizontal => (reflection_line + 1)..pattern.len(),
        Mode::Vertical => (reflection_line + 1)..pattern[0].len()
    };

    for (i, j) in range_i.rev().zip(range_j) {
        if !are_equal(i, j, pattern, mode, flip) { return false; }
    }

    true
}

fn are_equal(a: usize, b: usize, pattern: &[Vec<char>], mode: &Mode, flip: &Option<(usize, usize)>) -> bool {
    match *mode {
        Mode::Horizontal => pattern[a].iter().zip(pattern[b].iter()).enumerate().all(|(i, (char_a, char_b))| match_char(a, b, i, *char_a, *char_b, flip)),
        Mode::Vertical => pattern.iter().map(|line| line[a]).zip(pattern.iter().map(|line| line[b])).enumerate().all(|(i, (char_a, char_b))| match_char(a, b, i, char_a, char_b, flip))
    }
}

fn match_char(a: usize, b: usize, i: usize, char_a: char, char_b: char, flip: &Option<(usize, usize)>) -> bool {
    if flip.is_some() {
        let (j, k) = flip.unwrap();
        if (a == j || b == j) && k == i { return char_a != char_b; }
    }

    char_a == char_b
}