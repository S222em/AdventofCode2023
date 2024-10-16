use std::collections::{HashMap, HashSet};
use std::fs;

type Wires<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let wires = parse(&file);

    let disconnect = find_wires_to_disconnect(&wires).unwrap();

    let sizes = find_group_sizes(&wires, &disconnect);

    println!("{}", sizes.iter().product::<usize>());
}

fn find_group_sizes(wires: &Wires, disconnect: &[(&str, &str)]) -> Vec<usize> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut sizes: Vec<usize> = Vec::new();

    for wire in wires.keys() {
        if visited.contains(wire) { continue; }

        let connections = find_connections(wire, wires, disconnect);
        sizes.push(connections.len());
        visited.extend(&connections);
    }

    sizes
}

fn find_connections<'a>(start: &'a str, wires: &'a Wires, disconnect: &[(&str, &str)]) -> HashSet<&'a str> {
    let mut queue: Vec<&str> = Vec::from([start]);
    let mut visited: HashSet<&str> = HashSet::new();

    while !queue.is_empty() {
        let parent = queue.remove(0);

        if !visited.insert(parent) { continue; }

        let children = find_children(parent, wires);

        for child in children {
            if disconnect.contains(&(parent, child)) || disconnect.contains(&(child, parent)) { continue; }
            queue.push(child);
        }
    }

    visited
}

fn find_wires_to_disconnect<'a>(wires: &'a Wires) -> Option<Vec<(&'a str, &'a str)>> {
    for (i, start) in wires.keys().enumerate() {
        for end in wires.keys().skip(i + 1) {
            let result = recursive_disconnect(start, end, Vec::new(), wires);

            if result.is_some() { return result; }
        }
    }

    None
}

const TARGET_DISCONNECTED: usize = 3;

fn recursive_disconnect<'a>(start: &'a str, end: &'a str, disconnected: Vec<(&'a str, &'a str)>, wires: &'a Wires) -> Option<Vec<(&'a str, &'a str)>> {
    let path = find_path(start, end, &disconnected, wires);

    if path.is_none() && disconnected.len() == TARGET_DISCONNECTED { return Some(disconnected); }
    if disconnected.len() == TARGET_DISCONNECTED { return None; }

    let path = path.unwrap();
    if path.len() <= 2 { return None; }

    for (i, &node) in path.iter().enumerate().skip(1) {
        let prev = path[i - 1];
        let mut next_disconnected = disconnected.clone();
        next_disconnected.push((prev, node));

        let result = recursive_disconnect(start, end, next_disconnected, wires);
        if let Some(result) = result { return Some(result); }
    }

    None
}

fn find_path<'a>(start: &'a str, end: &'a str, disconnected: &[(&'a str, &'a str)], wires: &'a Wires) -> Option<Vec<&'a str>> {
    let mut queue: Vec<(&str, Vec<&str>)> = Vec::from([(start, Vec::new())]);
    let mut visited: HashSet<&str> = HashSet::new();

    while !queue.is_empty() {
        let (parent, mut path) = queue.remove(0);

        if parent == end {
            return Some(path);
        }
        if !visited.insert(parent) { continue; }

        path.push(parent);

        let children = find_children(parent, wires);

        for child in children {
            if disconnected.contains(&(parent, child)) { continue; }

            queue.push((child, path.clone()));
        }
    }

    None
}

fn find_children<'a>(parent: &'a str, wires: &'a Wires) -> Vec<&'a str> {
    let mut children = wires.get(parent).unwrap_or(&Vec::new()).clone();

    for (wire, wire_children) in wires.iter() {
        if !wire_children.contains(&parent) { continue; }
        children.push(wire);
    }

    children
}

fn parse(file: &str) -> Wires {
    let mut wires: Wires = HashMap::new();

    for line in file.lines() {
        let (parent, children) = line.split_once(": ").unwrap();

        let children: Vec<&str> = children.split(" ").collect();

        wires.insert(parent, children);
    }

    wires
}