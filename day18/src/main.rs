use std::fs;

type Edge = (isize, isize);

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lines: Vec<_> = file.lines().collect();

    let edges = find_edges(&lines);

    println!("{}", find_area(&edges));
}

fn find_area(edges: &[Edge]) -> usize {
    let interior = edges.windows(2).fold(0, |acc, windows| acc + (windows[0].0 * windows[1].1 - windows[0].1 * windows[1].0)).unsigned_abs() / 2;
    let boundary = edges.windows(2).fold(0, |acc, windows| acc + (windows[1].0 - windows[0].0 + windows[1].1 - windows[0].1).unsigned_abs());

    interior + boundary / 2 + 1
}

fn find_edges(lines: &[&str]) -> Vec<Edge> {
    let mut edge: Edge = (0, 0);
    let mut edges = vec![edge];

    for &line in lines {
        let (length, direction) = parse(line);

        match direction {
            0 => edge.1 += length,
            1 => edge.0 += length,
            2 => edge.1 -= length,
            3 => edge.0 -= length,
            _ => {}
        }

        edges.push(edge);
    }

    edges
}

fn parse(line: &str) -> (isize, usize) {
    let (length, direction) = line.split_whitespace().last().unwrap().strip_prefix("(#").unwrap().strip_suffix(")").unwrap().split_at(5);
    let length: isize = isize::from_str_radix(length, 16).unwrap();
    let direction: usize = usize::from_str_radix(direction, 16).unwrap();

    (length, direction)
}
