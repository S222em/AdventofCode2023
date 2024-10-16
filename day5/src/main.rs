use std::cmp::{max, min};
use std::fs;
use std::ops::Range;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let maps: Vec<_> = file.split("\n\n").collect();

    let (&seeds_line, maps) = maps.split_first().unwrap();

    let maps: Vec<Vec<[Range<usize>; 2]>> = maps.iter().map(|map| to_ranges(&map)).collect();

    let seeds: Vec<_> = seeds_line.split_whitespace().skip(1).collect();

    let mut lowest_location: usize = 0;

    for info in seeds.chunks(2) {
        let start: usize = info[0].parse().unwrap();
        let length: usize = info[1].parse().unwrap();
        let range = start..(start + length);

        let mut previous: Vec<Range<usize>> = vec![range];

        for map in maps.iter() {
            previous = find(&previous, map);
        }

        let lowest_start = previous.iter().map(|range| range.start).min().unwrap();

        if lowest_location == 0 { lowest_location = lowest_start } else { lowest_location = min(lowest_location, lowest_start) }
    }

    println!("{}", lowest_location);
}

fn find(ranges: &Vec<Range<usize>>, map: &Vec<[Range<usize>; 2]>) -> Vec<Range<usize>> {
    let mut found: Vec<Range<usize>> = Vec::new();

    for range in ranges {
        let overlap_with = map.iter().find(|[_, source]| overlap(range, source));

        if overlap_with.is_none() {
            found.push(range.clone());
            continue;
        }

        let [destination, source] = overlap_with.unwrap();

        let mut remainder = Vec::new();

        let fitted = fit(range, source, &mut remainder);
        let transformed = transform(&fitted, source, destination);

        found.push(transformed);

        if !remainder.is_empty() { found.extend(find(&remainder, map)) }
    }

    found
}

fn overlap(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.len() + b.len() > max(a.end, b.end) - min(a.start, b.start)
}

fn fit(a: &Range<usize>, b: &Range<usize>, remainder: &mut Vec<Range<usize>>) -> Range<usize> {
    let mut range = a.clone();

    if b.start > a.start {
        remainder.push(a.start..b.start);
        range.start = b.start
    }

    if a.end > b.end {
        remainder.push(b.end..a.end);
        range.end = b.end;
    }

    range
}

fn transform(a: &Range<usize>, b: &Range<usize>, c: &Range<usize>) -> Range<usize> {
    let start_diff = a.start as isize - b.start as isize;
    let end_diff = a.end as isize - b.end as isize;

    let start: usize = (c.start as isize + start_diff) as usize;
    let end: usize = (c.end as isize + end_diff) as usize;

    start..end
}

fn to_ranges(map: &str) -> Vec<[Range<usize>; 2]> {
    let lines: Vec<_> = map.split("\n").skip(1).collect();

    let mut ranges = Vec::new();

    for line in lines {
        let info: Vec<_> = line.split_whitespace().collect();
        let destination: usize = info[0].parse().unwrap();
        let source: usize = info[1].parse().unwrap();
        let length: usize = info[2].parse().unwrap();

        let source_range = source..(source + length);
        let destination_range = destination..(destination + length);

        ranges.push([destination_range, source_range]);
    }

    ranges
}
