fn main() {
    let input = include_str!("../input");

    let sum: usize = input.lines().map(|line| {
        let without_game = line.replace("Game ", "");

        let clean = &without_game[without_game.find(": ").unwrap() + 2..];
        let sets: Vec<_> = clean.split(';').collect();

        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        for set in sets {
            let components = set.split(',');
            for component in components {
                let (amount, color) = parse_component(component);

                if color == "red" {
                    max_red = max_red.max(amount);
                }
                if color == "green" {
                    max_green = max_green.max(amount);
                }
                if color == "blue" {
                    max_blue = max_blue.max(amount);
                }
            }
        }



        max_blue * max_green * max_red
    }).sum();

    println!("{}", sum);

}

fn parse_component(component: &str) -> (usize, &str) {
    let parts: Vec<_> = component.trim().split_ascii_whitespace().collect();

    (parts[0].parse().unwrap(), parts[1])
}
