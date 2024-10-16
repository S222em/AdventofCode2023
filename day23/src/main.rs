use std::collections::{HashMap, HashSet};
use std::fs;

type Grid = Vec<Vec<char>>;
type Coordinate = (usize, usize);
type Direction = (isize, isize);
type Corner = Vec<(Coordinate, usize)>;
type Corners = HashMap<Coordinate, Corner>;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let grid: Grid = file.lines().map(|line| line.chars().collect()).collect();

    let corners = explore_all(&grid);

    let longest_path = find_longest_path(&corners, &grid);

    println!("{:?}", longest_path);
}

fn find_longest_path(corners: &Corners, grid: &Grid) -> usize {
    let start: Coordinate = (0, grid[0].iter().position(|char| *char == '.').unwrap());
    let end: Coordinate = (grid.len() - 1, grid[grid.len() - 1].iter().position(|char| *char == '.').unwrap());

    next(&start, 0, &end, corners, HashSet::from([start]))
}

fn next(position: &Coordinate, steps: usize, end: &Coordinate, corners: &Corners, visited: HashSet<Coordinate>) -> usize {
    let mut most_steps = 0;

    let connected_corners = corners.get(position).unwrap();

    for (corner_position, corner_steps) in connected_corners {
        if visited.contains(corner_position) { continue; }

        if corner_position == end { return steps + corner_steps; }

        let mut visited_clone = visited.clone();
        visited_clone.insert(*corner_position);

        let steps = next(corner_position, steps + corner_steps, end, corners, visited_clone);

        if steps > most_steps { most_steps = steps; }
    }

    most_steps
}

fn explore_all(grid: &Grid) -> Corners {
    let mut corners: Corners = HashMap::new();
    let start: Coordinate = (0, grid[0].iter().position(|char| *char == '.').unwrap());
    corners.insert(start, Vec::new());

    let mut visited: HashSet<Coordinate> = HashSet::new();
    visited.insert(start);

    explore(start, (1, 0), grid, &mut visited, &mut corners);

    corners
}

fn explore(last_corner_position: Coordinate, mut direction: Direction, grid: &Grid, visited: &mut HashSet<Coordinate>, corners: &mut Corners) {
    let mut steps: usize = 0;
    let mut position: Coordinate = last_corner_position;

    loop {
        steps += 1;

        let potential_position = step(&position, &direction, grid);
        if potential_position.is_none() { break; }
        position = potential_position.unwrap();

        if corners.contains_key(&position) {
            update_corners(&position, &last_corner_position, steps, corners);
            break;
        }

        if visited.contains(&position) { break; }

        visited.insert(position);

        let paths = find_surrounding_paths(&position, &direction, grid);

        if paths.is_empty() && position.0 != grid.len() - 1 { break; }

        if paths.len() >= 2 || position.0 == grid.len() - 1 {
            update_corners(&position, &last_corner_position, steps, corners);

            for path in paths {
                explore(position, path, grid, visited, corners);
            }

            break;
        }

        direction = paths[0];
    };
}

fn update_corners(current: &Coordinate, previous: &Coordinate, steps: usize, corners: &mut Corners) {
    let corner = corners.entry(*current).or_default();
    corner.push((*previous, steps));

    let previous_corner = corners.entry(*previous).or_default();
    previous_corner.push((*current, steps));
}

fn find_surrounding_paths(position: &Coordinate, direction: &Direction, grid: &Grid) -> Vec<Direction> {
    let mut paths: Vec<Direction> = Vec::new();

    for possible_direction in [*direction, (-direction.1, direction.0), (direction.1, -direction.0)] {
        let possible_position = step(position, &possible_direction, grid);

        if possible_position.is_some() && grid[possible_position.unwrap().0][possible_position.unwrap().1] != '#' {
            paths.push(possible_direction);
        }
    }

    paths
}

fn step(position: &Coordinate, direction: &Direction, grid: &Grid) -> Option<Coordinate> {
    let unverified_position: Direction = (position.0 as isize + direction.0, position.1 as isize + direction.1);
    if unverified_position.0 < 0 || unverified_position.1 < 0 || unverified_position.0.unsigned_abs() >= grid.len() || unverified_position.1.unsigned_abs() >= grid[0].len() { return None; }
    let next_position = (unverified_position.0.unsigned_abs(), unverified_position.1.unsigned_abs());

    Some(next_position)
}