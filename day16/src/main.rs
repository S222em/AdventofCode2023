use std::collections::HashSet;
use std::fs;

type Position = (usize, usize);
type Direction = (isize, isize);

type Contraption = Vec<Vec<char>>;
type Beam = (Position, Direction);
type Beams = Vec<Beam>;
type Visited = HashSet<Beam>;
type Energized<'a> = HashSet<&'a Position>;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let contraption: Contraption = file.lines().map(|line| line.chars().collect()).collect();

    let start_beams = get_start_beams(&contraption);

    let mut max_energized: usize = 0;

    for start_beam in start_beams {
        let energized = find_amount_of_energized_tiles(start_beam, &contraption);

        if energized > max_energized { max_energized = energized }
    }


    println!("{}", max_energized)
}

fn get_start_beams(contraption: &Contraption) -> Beams {
    let mut beams = Vec::new();

    // Add all the possible starting beams on the left and right of the contraption
    for y in 0..contraption.len() {
        beams.push(((0, y), (1, 0)));
        beams.push(((contraption[0].len() - 1, y), (-1, 0)));
    }

    // Add all the possible starting beams on the front and back of the contraption
    for x in 0..contraption[0].len() {
        beams.push(((x, 0), (0, 1)));
        beams.push(((x, contraption.len() - 1), (0, -1)));
    }

    beams
}

fn find_amount_of_energized_tiles(start_beam: Beam, contraption: &Contraption) -> usize {
    let mut beams: Beams = vec![start_beam];
    let mut visited: Visited = HashSet::new();

    visited.insert(start_beam);

    while !beams.is_empty() {
        let mut removed: usize = 0;

        for mut i in 0..beams.len() {
            // Account for removed beams
            i -= removed;

            if step(i, &mut beams, contraption, &mut visited) { continue; }

            // Remove the beam as it's no longer valid, or it's path has been visited before
            beams.remove(i);
            removed += 1;
        }
    }

    // Create a set containing all visited positions (energized positions) (no duplicates)
    let energized: Energized = HashSet::from_iter(visited.iter().map(|(position, _)| position));

    energized.len()
}

fn step(i: usize, beams: &mut Beams, contraption: &Contraption, visited: &mut Visited) -> bool {
    // Get the content of the tile
    let tile = contraption[beams[i].0.1][beams[i].0.0];

    // Change the direction of the beam according to the contents of the tile
    match tile {
        '/' => if beams[i].1.0 != 0 { beams[i].1 = (beams[i].1.1, -beams[i].1.0) } else { beams[i].1 = (-beams[i].1.1, beams[i].1.0) },
        '\\' => if beams[i].1.0 != 0 { beams[i].1 = (-beams[i].1.1, beams[i].1.0) } else { beams[i].1 = (beams[i].1.1, -beams[i].1.0) },
        tile if (tile == '|' && beams[i].1.0 != 0) || (tile == '-' && beams[i].1.1 != 0) => {
            beams[i].1 = (-beams[i].1.1, beams[i].1.0);
            // Split the beam and add it to the list of beams if the next position it will be on is valid and not visited before
            let clone: Beam = (beams[i].0, (-beams[i].1.0, -beams[i].1.1));
            if let Some(clone_next_position) = next(&clone.0, &clone.1, contraption, visited) { beams.push((clone_next_position, clone.1)); }
        }
        _ => {}
    };

    // Move the beam to it's next position if it's new position is valid and not visited before
    if let Some(next_position) = next(&beams[i].0, &beams[i].1, contraption, visited) {
        beams[i].0 = next_position;
        return true;
    }

    false
}

fn next(position: &Position, direction: &Direction, contraption: &Contraption, visited: &mut Visited) -> Option<Position> {
    let x = position.0 as isize + direction.0;
    let y = position.1 as isize + direction.1;

    // Check if x and y are in range
    if x < 0 || x.unsigned_abs() >= contraption[0].len() || y < 0 || y.unsigned_abs() >= contraption.len() { return None; }

    let position = (x.unsigned_abs(), y.unsigned_abs());

    // If we already visited this position and direction before there is no need to do so again
    if !visited.insert((position, *direction)) { return None; }

    Some(position)
}
