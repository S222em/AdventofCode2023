use std::fs;

type Point = [usize; 2];

type Table = Vec<Vec<char>>;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let table: Table = file.split("\n").map(|line| line.chars().collect()).collect();

    let galaxies = find_galaxies(&table);

    let mut sum: usize = 0;

    for (i, a) in galaxies.iter().enumerate() {
        for (j, b) in galaxies.iter().enumerate() {
            if j <= i { continue; }

            let distance = distance_between(a, b);

            sum += distance;
        }
    }

    println!("Sum: {}", sum);
}

fn distance_between(a: &Point, b: &Point) -> usize {
    (b[0] as isize - a[0] as isize).unsigned_abs() + (b[1] as isize - a[1] as isize).unsigned_abs()
}

fn find_galaxies(table: &Table) -> Vec<Point> {
    let mut points = Vec::new();

    let rows = find_row_expansion(table);
    let columns = find_column_expansion(table);

    for (x, line) in table.iter().enumerate() {
        for (y, char) in line.iter().enumerate() {
            let x_correction = rows.iter().fold(0, |acc, row| acc + if (0..x).contains(row) { 999_999 } else { 0 });
            let y_correction = columns.iter().fold(0, |acc, column| acc + if (0..y).contains(column) { 999_999 } else { 0 });

            if *char == '#' { points.push([x + x_correction, y + y_correction]) }
        }
    }

    points
}

fn find_row_expansion(table: &Table) -> Vec<usize> {
    let mut line_expansions = Vec::new();

    for (i, row) in table.iter().enumerate() {
        if row.contains(&'#') { continue; }

        line_expansions.push(i)
    }

    line_expansions
}

fn find_column_expansion(table: &Table) -> Vec<usize> {
    let mut column_expansions = Vec::new();

    for i in 0..table[0].len() {
        if table.iter().any(|row| row[i] == '#') { continue; }

        column_expansions.push(i)
    }

    column_expansions
}


