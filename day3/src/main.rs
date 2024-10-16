use std::fs;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lines: Vec<_> = file.split("\n").collect();

    let stars = find_stars(&lines);

    let number_lines = find_numbers(&lines);

    let mut sum: u32 = 0;

    for [i, j] in stars {
        let mut range = i..(i + 1);
        if i > 0 { range.start = i - 1 }
        if i < lines[i].len() - 1 { range.end = i + 2 }

        let mut ratio: u32 = 1;
        let mut count: u32 = 0;

        for k in range {
            let number_line = &number_lines[k];

            for (l, str) in number_line {
                let range = *l..(l + str.len());

                if range.contains(&j) || (j != 0 && range.contains(&(j - 1))) || range.contains(&(j + 1)) {
                    let number: u32 = str.parse().unwrap();
                    ratio *= number;
                    count += 1;
                }
            }
        }

        if count > 1 { sum += ratio }
    }

    println!("{}", sum);
}

fn find_stars(lines: &[&str]) -> Vec<[usize; 2]> {
    let mut stars: Vec<[usize; 2]> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '*' { stars.push([i, j]); }
        }
    }

    stars
}

fn find_numbers<'a>(lines: &[&'a str]) -> Vec<Vec<(usize, &'a str)>> {
    let mut number_lines: Vec<_> = Vec::new();

    for line in lines.iter() {
        let mut number_line: Vec<(usize, &str)> = Vec::new();
        let mut start_j = 0;

        for (j, char) in line.chars().enumerate() {
            if !char.is_numeric() {
                if start_j != j {
                    let str = &line[start_j..j];
                    number_line.push((start_j, str));
                }

                start_j = j + 1;
            }
        }

        if start_j != line.len() {
            let str = &line[start_j..line.len()];
            number_line.push((start_j, str));
        }

        number_lines.push(number_line);
    }

    number_lines
}