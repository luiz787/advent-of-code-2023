use std::collections::HashMap;

#[derive(Debug)]
struct Instructions {
    directions: Vec<char>,
    documents: HashMap<String, (String, String)>,
}

fn main() {
    let instructions = parse_input(include_str!("../input"));

    let mut curr = "AAA";

    let mut directions_iterator = instructions.directions.iter().cloned().cycle();

    let mut steps = 0;
    while curr != "ZZZ" {
        let (left, right) = instructions.documents.get(curr).unwrap();

        let direction_to_follow = directions_iterator.next().unwrap();
        if direction_to_follow == 'R' {
            curr = right;
        } else {
            curr = left;
        }

        steps += 1;
    }

    println!("{}", steps);
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
