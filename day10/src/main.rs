use std::fs;
type Point = [usize; 2];
type Translate = [isize; 2];
type Matrix<'a> = Vec<Vec<&'a str>>;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let matrix: Matrix = file.split("\n").map(|str| str.split("").skip(1).collect()).collect();

    let points = find_path(&matrix).unwrap();

    let area: usize = points.windows(2).fold(0, |acc, window| acc + (window[0][0] * window[1][1]) as isize - (window[0][1] * window[1][0]) as isize).unsigned_abs() / 2;

    let count: usize = area - (points.len() / 2) + 1;

    println!("{}", count);
}

fn find_path(matrix: &Matrix) -> Option<Vec<Point>> {
    let point = get_start(matrix)?;

    for translate in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
        let mut points = vec![point];

        let correct_path = explore(matrix, point, translate, &mut points);

        points.push(point);

        if correct_path { return Some(points); }
    }

    None
}

fn explore(matrix: &Matrix, from: Point, translate: Translate, points: &mut Vec<Point>) -> bool {
    let x: isize = from[0] as isize + translate[0];
    let y: isize = from[1] as isize + translate[1];
    if x.is_negative() || y.is_negative() { return false; }

    let name = matrix[x as usize][y as usize];
    if name == "S" { return true; }
    if !connects(name, &translate) { return false; }

    points.push([x as usize, y as usize]);

    let flipped = flip(name, &translate);

    explore(matrix, [x as usize, y as usize], flipped, points)
}
fn connects(name: &str, translate: &Translate) -> bool {
    match name {
        "|" => translate[1] == 0,
        "-" => translate[0] == 0,
        "L" => !(translate[0] == -1 || translate[1] == 1),
        "J" => !(translate[0] == -1 || translate[1] == -1),
        "7" => !(translate[0] == 1 || translate[1] == -1),
        "F" => !(translate[0] == 1 || translate[1] == 1),
        _ => false
    }
}

fn flip(name: &str, translate: &Translate) -> Translate {
    match name {
        "|" => [translate[0], 0],
        "-" => [0, translate[1]],
        "L" => [if translate[0] == 0 { -1 } else { 0 }, if translate[1] == 0 { 1 } else { 0 }],
        "J" => [if translate[0] == 0 { -1 } else { 0 }, if translate[1] == 0 { -1 } else { 0 }],
        "7" => [if translate[0] == 0 { 1 } else { 0 }, if translate[1] == 0 { -1 } else { 0 }],
        "F" => [if translate[0] == 0 { 1 } else { 0 }, if translate[1] == 0 { 1 } else { 0 }],
        _ => [0, 0]
    }
}

fn get_start(matrix: &Matrix) -> Option<Point> {
    for (x, row) in matrix.iter().enumerate() {
        for (y, item) in row.iter().enumerate() {
            if *item == "S" { return Some([x, y]); }
        }
    }

    None
}
