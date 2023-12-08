use std::collections::HashMap;

#[derive(Debug)]
struct Instructions {
    directions: Vec<char>,
    documents: HashMap<String, (String, String)>,
}

fn main() {
    let instructions = parse_input(include_str!("../input"));

    let starting_nodes: Vec<_> = instructions
        .documents
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect();


    let mut steps_vector = Vec::new();

    for starting_node in starting_nodes {
        let mut directions_iterator = instructions.directions.iter().cloned().cycle();

        let mut curr = starting_node;
        let mut steps: u64 = 0;
        while !curr.ends_with("Z"){
            let (left, right) = instructions.documents.get(curr).unwrap();

            let direction_to_follow = directions_iterator.next().unwrap();
            if direction_to_follow == 'R' {
                curr = right;
            } else {
                curr = left;
            }

            steps += 1;
        }

        steps_vector.push(steps);
    }

    let result = lcm(steps_vector.as_slice());



    println!("{}", result);
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn parse_input(input: &str) -> Instructions {
    let (directions, documents) = input.split_once("\n\n").unwrap();

    let documents: Vec<_> = documents
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" = ").unwrap();

            let to = to.replace("(", "");
            let to = to.replace(")", "");
            let (left, right) = to.split_once(", ").unwrap();

            (from.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();
    let mut documents_map = HashMap::new();

    for (from, (left, right)) in documents {
        documents_map.insert(from, (left, right));
    }

    Instructions {
        directions: directions.chars().collect(),
        documents: documents_map,
    }
}
