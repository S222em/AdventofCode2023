extern crate core;

use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

type Coordinate = (usize, usize, usize);
type Brick = (RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>);

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let mut falling_bricks: Vec<Brick> = parse(&file);

    falling_bricks.sort_by(|(_, _, a), (_, _, b)| a.start().cmp(b.start()));

    let mut bricks: Vec<Brick> = Vec::new();

    for falling_brick in falling_bricks.into_iter() {
        let brick = move_brick_to_ground(falling_brick, &bricks);

        bricks.push(brick);
    }

    let support = map_brick_supports(&bricks);

    let most_destruction = find_total_destruction(&support, &bricks);

    println!("{:?}", most_destruction);
}

fn find_total_destruction(support: &HashMap<usize, Vec<usize>>, bricks: &[Brick]) -> usize {
    let mut total_destruction = 0;

    for i in support.keys() {
        let destruction = find_destruction(*i, bricks, support);

        total_destruction += destruction;
    }

    total_destruction
}

fn find_destruction(i: usize, bricks: &[Brick], support: &HashMap<usize, Vec<usize>>) -> usize {
    let i_supporting = support.get(&i).unwrap();
    if i_supporting.is_empty() { return 0; }

    let mut queue: Vec<usize> = i_supporting.clone();
    let mut destroyed: Vec<usize> = Vec::from([i]);

    while !queue.is_empty() {
        let (lowest, _) = queue.iter().enumerate().min_by(|(_, &a), (_, &b)| bricks[a].2.start().cmp(bricks[b].2.start())).unwrap();
        let j = queue.remove(lowest);

        let has_other_supports = support.iter().any(|(k, k_supporting)| !destroyed.contains(k) && k_supporting.contains(&j));

        if has_other_supports { continue; }

        destroyed.push(j);

        let j_supporting = support.get(&j).unwrap();
        for j_supported in j_supporting {
            if queue.contains(j_supported) { continue; }
            queue.push(*j_supported);
        }
    }

    destroyed.len() - 1
}

fn map_brick_supports(bricks: &[Brick]) -> HashMap<usize, Vec<usize>> {
    let mut map = HashMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        let mut supporting: Vec<usize> = Vec::new();

        for (j, brick_above) in bricks.iter().enumerate().skip(i + 1) {
            if *brick_above.2.start() != brick.2.end() + 1 { continue; }

            if does_range_overlap(&brick.0, &brick_above.0) && does_range_overlap(&brick.1, &brick_above.1) {
                supporting.push(j);
            }
        }

        map.insert(i, supporting);
    }

    map
}

fn move_brick_to_ground(brick: Brick, bricks: &[Brick]) -> Brick {
    let bricks_in_path: Vec<&Brick> = bricks.iter().filter(|(x, y, _)| {
        does_range_overlap(&brick.0, x) && does_range_overlap(&brick.1, y)
    }).collect();

    let brick_z_len = brick.2.end() - brick.2.start();

    if bricks_in_path.is_empty() { return (brick.0, brick.1, 1..=(1 + brick_z_len)); }

    let highest_brick = bricks_in_path.iter().max_by(|(_, _, a), (_, _, b)| a.end().cmp(b.end())).unwrap();

    (brick.0, brick.1, (highest_brick.2.end() + 1)..=(highest_brick.2.end() + 1 + brick_z_len))
}

fn get_axis_range(a: usize, b: usize) -> RangeInclusive<usize> {
    min(a, b)..=max(a, b)
}

fn does_range_overlap(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

fn parse(file: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();

    for line in file.lines() {
        let (left, right) = line.split_once("~").unwrap();
        let left_coordinate = parse_coordinate(left);
        let right_coordinate = parse_coordinate(right);

        bricks.push((get_axis_range(left_coordinate.0, right_coordinate.0), get_axis_range(left_coordinate.1, right_coordinate.1), get_axis_range(left_coordinate.2, right_coordinate.2)))
    }

    bricks
}

fn parse_coordinate(coordinate: &str) -> Coordinate {
    let mut split = coordinate.split(",");

    (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
}
