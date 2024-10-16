use std::fs;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lines: Vec<_> = file.split("\n").collect();

    let time: usize = parse(lines[0]);
    let distance: usize = parse(lines[1]);

    let mut number: usize = 0;

    for speed in 1..(time / 2) {
        let time_left = time - speed;

        let distance_covered = speed * time_left;

        if distance_covered > distance {
            number += time - speed * 2 + 1;
            break;
        }
    }

    println!("{}", number);
}

fn parse(line: &str) -> usize {
    line.split_whitespace().skip(1).collect::<Vec<_>>().join("").parse().unwrap()
}

