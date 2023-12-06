#[derive(Debug)]
struct Race {
    time: usize,
    distance_to_beat: usize,
}

fn main() {
    let race = parse_input(include_str!("../input"));
    
    println!("{}", solve_with_equation(&race));
}

fn solve_with_equation(race: &Race) -> usize {
    // X * (T-X) = D
    // -X² + TX - D = 0

    let a: i64 = -1;
    let b: f64 = race.time as f64;
    let c: i64 = -1 * race.distance_to_beat as i64;

    let delta = b.powi(2) - (4*a*c) as f64;

    let first_root = (-b + f64::sqrt(delta)) / (2*a) as f64;
    let second_root = (-b - f64::sqrt(delta)) / (2*a) as f64;


    let first_root = first_root.ceil() as usize;
    let second_root = second_root.floor() as usize;

    return second_root - first_root + 1;
}

fn parse_input(input: &str) -> Race {
    let lines: Vec<_> = input.lines().collect();
    let times:Vec<usize> = lines[0]
        .replace("Time:", "")
        .replace(" ", "")
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.trim().parse().unwrap())
        .collect();

    let distances: Vec<usize> = lines[1]
        .replace("Distance:", "")
        .replace(" ", "")
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.trim().parse().unwrap())
        .collect();

    times.iter()
        .zip(distances)
        .map(|(a, b) |Race {time: *a, distance_to_beat: b})
        .next()
        .unwrap()
}
