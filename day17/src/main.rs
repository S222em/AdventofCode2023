use std::collections::HashSet;
use std::fs;

type City = Vec<Vec<usize>>;
type Block = (usize, usize);
type Direction = usize;
type Transform = (isize, isize);
type HeatLoss = usize;
type State = (Block, Direction, HeatLoss);
type Queue = Vec<State>;
type Visit = (Block, Direction);
type Visited = HashSet<Visit>;

const START_BLOCK: Block = (0, 0);
const START_STATES: [State; 2] = [(START_BLOCK, 0, 0), (START_BLOCK, 1, 0)];
const MIN_STEPS: usize = 4;
const MAX_STEPS: usize = 10;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let city: City = file.lines().map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as usize).collect()).collect();

    let target: Block = (city[0].len() - 1, city.len() - 1);

    let min_heat_loss = find_minimized_heat_loss(&city, &target);

    println!("{}", min_heat_loss.unwrap_or(0))
}

fn find_minimized_heat_loss(city: &City, target: &Block) -> Option<usize> {
    let mut visited: Visited = HashSet::new();
    let mut queue: Queue = Vec::from(START_STATES);

    while !queue.is_empty() {
        let (i, _) = queue.iter().enumerate().min_by_key(|(_, state)| state.2)?;
        let state = queue.remove(i);

        if state.0 == *target { return Some(state.2); }

        if !visited.contains(&(state.0, state.1)) {
            visited.insert((state.0, state.1));
            add_next_to_queue(&state, &mut queue, city);
        }
    }

    None
}

fn add_next_to_queue(state: &State, queue: &mut Queue, city: &City) {
    let transformations: [Transform; 2] = if state.1 == 0 { [(1, 0), (-1, 0)] } else { [(0, 1), (0, -1)] };

    for transformation in transformations {
        let mut heat_loss = state.2;

        for offset in 1..=MAX_STEPS {
            let next_block = transform(&state.0, &transformation, offset, city);
            if next_block.is_none() { break; }
            let next_block = next_block.unwrap();

            heat_loss += city[next_block.0][next_block.1];

            if offset < MIN_STEPS { continue; }

            let next_state: State = (next_block, if state.1 == 0 { 1 } else { 0 }, heat_loss);
            queue.push(next_state);
        }
    }
}

fn transform(block: &Block, transform: &Transform, offset: usize, city: &City) -> Option<Block> {
    let row = block.0 as isize + offset as isize * transform.0;
    let column = block.1 as isize + offset as isize * transform.1;

    if row < 0 || row.unsigned_abs() >= city.len() || column < 0 || column.unsigned_abs() >= city[0].len() { return None; }

    let block = (row.unsigned_abs(), column.unsigned_abs());

    Some(block)
}
