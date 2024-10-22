use std::fs;

type PositionOrVelocity = (f64, f64, f64);
type HailStone = (PositionOrVelocity, PositionOrVelocity);

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let hailstones = parse(&file);

    let result = find_stone_velocity_and_position(&hailstones);

    if let Some(result) = result {
        println!("{}", result.0.0 + result.0.1 + result.0.2)
    }
}

fn find_stone_velocity_and_position(hailstones: &[HailStone]) -> Option<HailStone> {
    let hailstone_a = &hailstones[0];
    let hailstone_b = &hailstones[1];
    let hailstone_c = &hailstones[2];

    for svx in -1000..1000 {
        'outer: for svy in -1000..1000 {
            let svx = svx as f64;
            let svy = svy as f64;

            let avx = hailstone_a.1.0 - svx;
            let avy = hailstone_a.1.1 - svy;

            let bvx = hailstone_b.1.0 - svx;
            let bvy = hailstone_b.1.1 - svy;

            let cvx = hailstone_c.1.0 - svx;
            let cvy = hailstone_c.1.1 - svy;

            let ab_intersection = find_xy_intersection(&(hailstone_a.0, (avx, avy, hailstone_a.1.2)), &(hailstone_b.0, (bvx, bvy, hailstone_b.1.2)));
            let ac_intersection = find_xy_intersection(&(hailstone_a.0, (avx, avy, hailstone_a.1.2)), &(hailstone_c.0, (cvx, cvy, hailstone_c.1.2)));

            if ab_intersection.is_none() || ac_intersection.is_none() { continue; }

            let (ab, t, s) = ab_intersection.unwrap();
            let (ac, u, _) = ac_intersection.unwrap();

            let t = t.round();
            let s = s.round();
            let u = u.round();

            if ab.0.round() != ac.0.round() || ab.1.round() != ac.1.round() || t != u { continue; }

            if t - s == 0.0 { continue; }
            let svz = ((hailstone_a.0.2 + hailstone_a.1.2 * t) - (hailstone_b.0.2 + hailstone_b.1.2 * s)) / (t - s);

            let sx = hailstone_a.0.0 + (hailstone_a.1.0 - svx) * t;
            let sy = hailstone_a.0.1 + (hailstone_a.1.1 - svy) * t;
            let sz = hailstone_a.0.2 + (hailstone_a.1.2 - svz) * t;

            let rock: HailStone = ((sx, sy, sz), (svx, svy, svz));

            for hailstone in hailstones.iter() {
                if find_xy_intersection(&rock, hailstone).is_none() { continue 'outer; }
            }

            return Some(rock);
        }
    }
    None
}

fn find_xy_intersection(a: &HailStone, b: &HailStone) -> Option<(PositionOrVelocity, f64, f64)> {
    let denominator: f64 = a.1.0 * -b.1.1 - -b.1.0 * a.1.1;
    if denominator == 0.0 { return None; }
    let inverse: f64 = 1.0 / denominator;

    let bx = b.0.0 - a.0.0;
    let by = b.0.1 - a.0.1;

    let t = inverse * -b.1.1 * bx + inverse * b.1.0 * by;
    let s = inverse * -a.1.1 * bx + inverse * a.1.0 * by;
    if t.is_sign_negative() || s.is_sign_negative() { return None; }

    let intersection_a = (a.0.0 + a.1.0 * t, a.0.1 + a.1.1 * t, a.0.2 + a.1.2 * t);

    Some((intersection_a, t, s))
}

fn parse(file: &str) -> Vec<HailStone> {
    let mut hailstones: Vec<HailStone> = Vec::new();

    for line in file.lines() {
        let (unparsed_position, unparsed_velocity) = line.split_once(" @ ").unwrap();

        hailstones.push((
            parse_position_or_velocity(unparsed_position),
            parse_position_or_velocity(unparsed_velocity),
        ))
    }

    hailstones
}

fn parse_position_or_velocity(unparsed_position_or_velocity: &str) -> PositionOrVelocity {
    let mut split = unparsed_position_or_velocity.split(", ");

    (
        split.next().unwrap().trim().parse().unwrap(),
        split.next().unwrap().trim().parse().unwrap(),
        split.next().unwrap().trim().parse().unwrap(),
    )
}
