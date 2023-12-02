fn main() {
    let input = include_str!("../input");

    let sum: usize = input.lines().map(|line| {
        let without_game = line.replace("Game ", "");
        let game_id: usize = without_game.replace("Game ", "")[0..without_game.find(":").unwrap()].parse().unwrap();

        let clean = &without_game[without_game.find(": ").unwrap() + 2..];
        let sets: Vec<_> = clean.split(';').collect();

        for set in sets {
            let components = set.split(',');
            for component in components {
                let (amount, color) = parse_component(component);

                if (color == "red" && amount > 12)
                || (color == "green" && amount > 13)
                || (color == "blue" && amount > 14) {
                    return 0
                }
            }
        }

        game_id
    }).sum();

    println!("{}", sum);

}

fn parse_component(component: &str) -> (usize, &str) {
    let parts: Vec<_> = component.trim().split_ascii_whitespace().collect();

    (parts[0].parse().unwrap(), parts[1])
}
