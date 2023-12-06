#[derive(Debug)]
struct MapEntry {
    dest_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<MapEntry>>,
}

fn main() {
    let input = include_str!("../input");
    let parsed_input = parse_input(input);

    let seeds = parsed_input.seeds.clone();

    let result = seeds.into_iter()
    .map(|v| map_number(v, &parsed_input.maps[1]))
    .map(|v| map_number(v, &parsed_input.maps[2]))
    .map(|v| map_number(v, &parsed_input.maps[3]))
    .map(|v| map_number(v, &parsed_input.maps[4]))
    .map(|v| map_number(v, &parsed_input.maps[5]))
    .map(|v| map_number(v, &parsed_input.maps[6]))
    .map(|v| map_number(v, &parsed_input.maps[7]))
    .min();

    println!("{:?}", result.unwrap());
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
        
        return mapped_value
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
        seeds: seeds,
        maps: maps,
    }
}
