use std::fs;

type Coordinate = (usize, usize);
type Transform = (isize, isize);

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Platform = Vec<Vec<char>>;

const ITERATIONS: usize = 1_000_000_000;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let mut platforms = parse(file);
    let platform = platforms.iter_mut().next().unwrap();

    let mut previous_platforms: Vec<Platform> = Vec::new();

    let mut i = 0;

    let load: usize = loop {
        if i == ITERATIONS { break calculate_load(platform, &Direction::North); }

        tilt(platform, &Direction::North);
        tilt(platform, &Direction::West);
        tilt(platform, &Direction::South);
        tilt(platform, &Direction::East);

        if let Some(cycle_start) = find_cycle(platform, &previous_platforms) {
            let cycle_len = i - cycle_start;

            let possible_cycles = (ITERATIONS - 1 - cycle_start) / cycle_len;

            i = cycle_start + (cycle_len * possible_cycles);

            let left = ITERATIONS - 1 - i;

            break calculate_load(&previous_platforms[cycle_start + left], &Direction::North);
        }

        previous_platforms.push(platform.clone());

        i += 1
    };

    println!("{}", load);
}

fn find_cycle(platform: &Platform, previous_platforms: &[Platform]) -> Option<usize> {
    let cycle_start = previous_platforms.iter().enumerate().find(|(_, platform_b)| equal_platforms(platform, platform_b));

    if let Some((cycle_start, _)) = cycle_start {
        return Some(cycle_start);
    }

    None
}

fn equal_platforms(a: &Platform, b: &Platform) -> bool {
    a.iter().zip(b.iter()).all(|(item_a, item_b)| {
        item_a.iter().zip(item_b.iter()).all(|(char_a, char_b)| char_a == char_b)
    })
}

fn tilt(platform: &mut Platform, direction: &Direction) {
    for x in get_range(direction, platform.len()) {
        for y in get_range(direction, platform[0].len()) {
            if platform[x][y] != 'O' { continue; }

            let (move_to_x, move_to_y) = roll((x, y), platform, direction);

            platform[x][y] = '.';
            platform[move_to_x][move_to_y] = 'O';
        }
    }
}

fn roll((mut x, mut y): Coordinate, platform: &Platform, direction: &Direction) -> Coordinate {
    loop {
        let next_coordinate = match direction {
            Direction::North => transform((x, y), (-1, 0), platform),
            Direction::East => transform((x, y), (0, 1), platform),
            Direction::South => transform((x, y), (1, 0), platform),
            Direction::West => transform((x, y), (0, -1), platform),
        };

        if next_coordinate.is_none() { break (x, y); }
        let (next_x, next_y) = next_coordinate.unwrap();

        if platform[next_x][next_y] != '.' { break (x, y); }

        (x, y) = (next_x, next_y)
    }
}

fn transform((x, y): Coordinate, (tx, ty): Transform, platform: &Platform) -> Option<Coordinate> {
    let x = x as isize + tx;
    let y = y as isize + ty;

    if x.is_negative() || x.unsigned_abs() >= platform.len() { return None; }
    if y.is_negative() || y.unsigned_abs() >= platform[0].len() { return None; }

    Some((x.unsigned_abs(), y.unsigned_abs()))
}

fn calculate_load(platform: &Platform, direction: &Direction) -> usize {
    let mut load: usize = 0;

    for x in get_range(direction, platform.len()) {
        for y in get_range(direction, platform[0].len()) {
            if platform[x][y] != 'O' { continue; }

            load += match direction {
                Direction::North => platform.len() - x,
                Direction::East => platform[0].len() - y,
                Direction::South => x + 1,
                Direction::West => y + 1,
            }
        }
    }

    load
}

fn get_range(direction: &Direction, length: usize) -> Box<dyn Iterator<Item=usize>> {
    let range = 0..length;

    if *direction == Direction::North || *direction == Direction::West { return Box::new(range); }

    Box::new(range.rev())
}

fn parse(file: String) -> Vec<Platform> {
    file.split("\n\n").map(|str_a| str_a.lines().map(|str_b| str_b.chars().collect()).collect()).collect()
}
