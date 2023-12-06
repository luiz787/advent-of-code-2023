#[derive(Debug)]
struct Race {
    time: usize,
    distance_to_beat: usize,
}

fn main() {
    let input = parse_input(include_str!("../input"));

    let mut ways_to_beat_races = Vec::new();
    
    for race in input {
        let mut ways = 0;
        for possible_time in 0..race.time {
            let distance = (race.time - possible_time) * possible_time;
            if distance > race.distance_to_beat {
                ways += 1;
            }
        }

        ways_to_beat_races.push(ways);
    }

    let result: usize = ways_to_beat_races
        .iter()
        .fold(1, |acc, v| acc * v);

    dbg!(result);
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<_> = input.lines().collect();
    let times:Vec<usize> = lines[0]
        .replace("Time:", "")
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.trim().parse().unwrap())
        .collect();

    let distances: Vec<usize> = lines[1]
        .replace("Distance:", "")
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.trim().parse().unwrap())
        .collect();

    times.iter()
        .zip(distances)
        .map(|(a, b) |Race {time: *a, distance_to_beat: b})
        .collect()
}
