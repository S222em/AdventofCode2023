use std::fs;

type PositionOrVelocity = (f64, f64, f64);
type HailStone = (PositionOrVelocity, PositionOrVelocity);

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let hailstones = parse(&file);

    let result = find_stone_velocity_and_position(&hailstones);

    if let Some(result) = result {
        println!("{}", result)
    }
}

fn find_stone_velocity_and_position(hailstones: &[HailStone]) -> Option<f64> {
    let hailstone_a = &hailstones[0];
    let hailstone_b = &hailstones[1];
    let hailstone_c = &hailstones[2];

    for t in 0..10000 {
        let hailstone_a_x = hailstone_a.0.0 + t as f64 * hailstone_a.1.0;
        let hailstone_a_y = hailstone_a.0.1 + t as f64 * hailstone_a.1.1;
        let hailstone_a_z = hailstone_a.0.2 + t as f64 * hailstone_a.1.2;

        for s in 0..10000 {
            if s - t == 0 {
                continue;
            }

            let hailstone_b_x = hailstone_b.0.0 + s as f64 * hailstone_b.1.0;
            let hailstone_b_y = hailstone_b.0.1 + s as f64 * hailstone_b.1.1;
            let hailstone_b_z = hailstone_b.0.2 + s as f64 * hailstone_b.1.2;

            let rock_ab_vx = (hailstone_b_x - hailstone_a_x) / (s - t) as f64;
            let rock_ab_vy = (hailstone_b_y - hailstone_a_y) / (s - t) as f64;
            let rock_ab_vz = (hailstone_b_z - hailstone_a_z) / (s - t) as f64;

            let rock_ab_v = (rock_ab_vx, rock_ab_vy, rock_ab_vz);

            for u in 0..10000 {
                if u - t == 0 {
                    continue;
                }

                let hailstone_c_x = hailstone_c.0.0 + u as f64 * hailstone_c.1.0;
                let hailstone_c_y = hailstone_c.0.1 + u as f64 * hailstone_c.1.1;
                let hailstone_c_z = hailstone_c.0.2 + u as f64 * hailstone_c.1.2;

                let rock_ac_vx = (hailstone_c_x - hailstone_a_x) / (u - t) as f64;
                let rock_ac_vy = (hailstone_c_y - hailstone_a_y) / (u - t) as f64;
                let rock_ac_vz = (hailstone_c_z - hailstone_a_z) / (u - t) as f64;

                let rock_ac_v = (rock_ac_vx, rock_ac_vy, rock_ac_vz);

                if rock_ab_v == rock_ac_v {
                    let rock_x = hailstone_a_x - t as f64 * rock_ab_vx;
                    let rock_y = hailstone_a_y - t as f64 * rock_ab_vy;
                    let rock_z = hailstone_a_z - t as f64 * rock_ab_vz;

                    return Some(rock_x + rock_y + rock_z);
                }
            }
        }
    }

    None
}

// fn find_xy_intersection(a: &HailStone, b: &HailStone) -> Option<(PositionOrVelocity, f64)> {
//     let denominator: f64 = a.1.0 * -b.1.1 - -b.1.0 * a.1.1;
//     if denominator == 0.0 { return None; }
//     let inverse: f64 = 1.0 / denominator;
//
//     let bx = b.0.0 - a.0.0;
//     let by = b.0.1 - a.0.1;
//
//     let t = inverse * -b.1.1 * bx + inverse * b.1.0 * by;
//     let s = inverse * -a.1.1 * bx + inverse * a.1.0 * by;
//     if t.is_sign_negative() || s.is_sign_negative() || t != s { return None; }
//
//     let intersection_a = (a.0.0 + a.1.0 * t, a.0.1 + a.1.1 * t, a.0.2 + a.1.2 * t);
//     //let intersection_b = (b.0.0 + b.1.0 * s, b.0.1 + b.1.1 * s, b.0.2 + b.1.2 * s);
//
//     //if intersection_a != intersection_b { return None; }
//
//     Some((intersection_a, t))
// }

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
