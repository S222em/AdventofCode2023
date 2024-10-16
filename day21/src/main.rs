use std::collections::{HashMap, HashSet};
use std::fs;

type Grid = Vec<Vec<char>>;
type Tile = (isize, isize);
type Tiles = HashSet<Tile>;
type Visited = HashMap<Tile, usize>;

// Step goal: 26501365
// Tile size: 131
// Center to edge: 65
// Amount of squares in any direction (except center): 202300
//

const STEPS: usize = 26501365;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let grid: Grid = file.lines().map(|line| line.chars().collect()).collect();

    let start = find_start(&grid);

    let amount_of_squares = find_amount_of_squares_in_any_direction(STEPS, &grid);

    let even_squares = find_amount_of_even_squares(amount_of_squares);
    let odd_squares = find_amount_of_odd_squares(amount_of_squares);

    let distances = find_distance_to_tiles_from(start, &grid);

    let even_tiles: usize = even_squares * distances.values().filter(|&distance| *distance % 2 == 0).count();
    let odd_tiles: usize = odd_squares * distances.values().filter(|&distance| *distance % 2 == 1).count();

    let even_corner_tiles: usize = amount_of_squares * distances.values().filter(|&distance| *distance % 2 == 0 && *distance > 65).count();
    let odd_corner_tiles: usize = (amount_of_squares + 1) * distances.values().filter(|&distance| *distance % 2 == 1 && *distance > 65).count();


    let tiles = even_tiles + odd_tiles + even_corner_tiles - odd_corner_tiles;

    println!("{}", tiles);
}

fn find_amount_of_squares_in_any_direction(max_distance: usize, grid: &Grid) -> usize {
    (max_distance - grid.len() / 2) / grid.len()
}

fn find_amount_of_odd_squares(amount_of_squares: usize) -> usize {
    (amount_of_squares + 1).pow(2)
}

fn find_amount_of_even_squares(amount_of_squares: usize) -> usize {
    amount_of_squares.pow(2)
}


fn find_start(grid: &Grid) -> Tile {
    for (r, row) in grid.iter().enumerate() {
        for (c, &char) in row.iter().enumerate() {
            if char == 'S' { return (r as isize, c as isize); }
        }
    }

    panic!("Starting point not found")
}

fn find_distance_to_tiles_from(center: Tile, grid: &Grid) -> Visited {
    let mut visited: Visited = HashMap::new();
    let mut tiles: Tiles = HashSet::from([center]);
    let mut distance: usize = 0;

    while !tiles.is_empty() {
        distance += 1;

        tiles = step(&tiles, grid, &mut visited);

        for tile in tiles.iter() {
            visited.insert(*tile, distance);
        }
    }

    visited
}

fn step(previous_tiles: &Tiles, grid: &Grid, visited: &mut Visited) -> Tiles {
    let mut tiles: Tiles = HashSet::new();

    for tile in previous_tiles.iter() {
        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let r = tile.0 + direction.0;
            let c = tile.1 + direction.1;

            if !is_in_bounds(r, grid.len()) || !is_in_bounds(c, grid[0].len()) { continue; }

            if grid[r.unsigned_abs()][c.unsigned_abs()] == '#' || visited.contains_key(&(r, c)) { continue; }

            tiles.insert((r, c));
        }
    }

    tiles
}

fn is_in_bounds(a: isize, len: usize) -> bool {
    a >= 0 && a < len as isize
}