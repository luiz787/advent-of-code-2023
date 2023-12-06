use std::{ops::Range, vec};

#[derive(Debug)]
struct MapEntry {
    dest_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<usize>>,
    maps: Vec<Vec<MapEntry>>,
}

fn main() {
    let input = include_str!("../input");
    let parsed_input = parse_input(input);

    let seeds = parsed_input.seeds.clone();

    let result = solve_smart(seeds, parsed_input);

    println!("{:?}", result);
}

fn deal_with_iteration(seed: &Range<usize>, mappers: &Vec<MapEntry>) -> Vec<Range<usize>> {
    let mut resulting_seeds = Vec::new();
    for map in mappers {
        for r in map_range(&seed, &map) {
            resulting_seeds.push(r);
        }
    }

    resulting_seeds
}

fn solve_smart(seeds: Vec<Range<usize>>, parsed_input: Almanac) -> usize {
    let mut cp = seeds.clone();

    for it_maps in &parsed_input.maps[1..] {
        let mut new = Vec::new();
        while !cp.is_empty() {
            let seed = cp.pop().unwrap();

            for map_entry in it_maps {
                let overlap_start = seed.start.max(map_entry.source_range_start);
                let overlap_end =
                    (seed.end).min(map_entry.source_range_start + map_entry.range_length);

                if overlap_start < overlap_end {
                    new.push(do_map(overlap_start, map_entry)..do_map(overlap_end, map_entry));
                    
                    if overlap_start > seed.start {
                        cp.push(seed.start..overlap_start);
                    }
                    if seed.end > overlap_end {
                        cp.push(overlap_end..seed.end);
                    }
                    break;
                } else {
                    new.push(seed.clone());
                }
            }
        }
        cp = new;
    }

    dbg!(&cp);

    cp
        .iter()
        .map(|seed| seed.start)
        .min()
        .unwrap()
}

fn map_range(seed: &Range<usize>, map_entry: &MapEntry) -> Vec<Range<usize>> {
    let mut resulting_ranges = Vec::new();
    let overlap_start = seed.start.max(map_entry.source_range_start);
    let overlap_end = (seed.end - 1).min(map_entry.source_range_start + map_entry.range_length);

    if overlap_end < overlap_start {
        return vec![seed.clone()];
    }

    if overlap_start > seed.start {
        resulting_ranges.push(map_entry.source_range_start..overlap_start);
        resulting_ranges.push(overlap_end..(map_entry.source_range_start + map_entry.range_length));
    }

    resulting_ranges.push(do_map(overlap_start, map_entry)..do_map(overlap_end, map_entry));

    resulting_ranges
}

fn solve_brute_force(seeds: Vec<Range<usize>>, parsed_input: Almanac) -> usize {
    seeds
        .into_iter()
        .map(|range| range.into_iter())
        .flatten()
        .fold(usize::MAX, |acc, seed| {
            let seed = map_number(seed, &parsed_input.maps[1]);
            let seed = map_number(seed, &parsed_input.maps[2]);
            let seed = map_number(seed, &parsed_input.maps[3]);
            let seed = map_number(seed, &parsed_input.maps[4]);
            let seed = map_number(seed, &parsed_input.maps[5]);
            let seed = map_number(seed, &parsed_input.maps[6]);
            let seed = map_number(seed, &parsed_input.maps[7]);

            return acc.min(seed);
        })
}

fn do_map(value: usize, map_entry: &MapEntry) -> usize {
    let delta_to_source_range_start = value - map_entry.source_range_start;

    let mapped_value = map_entry.dest_range_start + delta_to_source_range_start;

    return mapped_value;
}

fn map_number(value: usize, mappers: &Vec<MapEntry>) -> usize {
    for mapper in mappers {
        if value < mapper.source_range_start {
            continue;
        }

        if value > (mapper.source_range_start + mapper.range_length) {
            continue;
        }

        let delta_to_source_range_start = value - mapper.source_range_start;

        let mapped_value = mapper.dest_range_start + delta_to_source_range_start;

        return mapped_value;
    }

    value
}

fn parse_input(input: &str) -> Almanac {
    let mut lines = input.lines();

    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .replace("seeds: ", "")
        .split_ascii_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();
    let mut actual_seeds = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        let start_seed = seeds[i];
        let size = seeds[i + 1];

        actual_seeds.push(start_seed..(start_seed + size));

        i += 2;
    }

    let lines: Vec<_> = input.lines().collect();

    let mut i = 0;
    let mut maps = Vec::new();
    while i < lines.len() {
        let current_line = lines[i];
        if current_line.trim().is_empty() {
            i += 1;
            continue;
        }
        if current_line.chars().next().unwrap().is_alphabetic() {
            let mut j = 0;

            while i + 1 + j < lines.len()
                && !lines[i + 1 + j].is_empty()
                && lines[i + 1 + j].chars().next().unwrap().is_numeric()
            {
                j += 1;
            }

            let mut entries = Vec::new();
            for k in i + 1..i + j + 1 {
                let map_entry_values: Vec<_> = lines[k]
                    .trim()
                    .split_ascii_whitespace()
                    .map(|v| v.trim().parse().unwrap())
                    .collect();

                entries.push(MapEntry {
                    dest_range_start: map_entry_values[0],
                    source_range_start: map_entry_values[1],
                    range_length: map_entry_values[2],
                });
            }
            maps.push(entries);
            i += j + 1;
        } else {
        }
    }

    Almanac {
        seeds: actual_seeds,
        maps: maps,
    }
}
