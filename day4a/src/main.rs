#[derive(Debug)]
struct Card {
    number: usize,
    winning_numbers: Vec<usize>,
    numbers_you_have: Vec<usize>
}

fn main() {
    let input = include_str!("../input");
    let cards = parse_input(input);

    let mut result = 0;
    for card in cards {
        let mut amount_of_winning_numbers = 0;
        for num in card.winning_numbers {
            if card.numbers_you_have.contains(&num) {
                amount_of_winning_numbers += 1;
            }
        }

        if amount_of_winning_numbers > 0 {
            amount_of_winning_numbers = 2_u32.pow(amount_of_winning_numbers - 1);
        }

        result += amount_of_winning_numbers;
    }


    println!("{:?}", result);
}

fn parse_input(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    
    for line in input.lines() {
        let line_without_prefix = line.replace("Card ", "");
        let mut parts = line_without_prefix.trim().split(":");
        let card_number = parts.next().unwrap();

        let mut numbers = parts.next().unwrap().split(" | ");
        let winning_numbers = str_to_vec_usize(numbers.next().unwrap());

        let numbers_you_have = str_to_vec_usize(numbers.next().unwrap());

        cards.push(Card {
            number: card_number.parse().unwrap(),
            winning_numbers,
            numbers_you_have
        });
    }

    cards
}

fn str_to_vec_usize(s: &str) -> Vec<usize> {
    let mut result = Vec::new();

    for value in s.trim().split_ascii_whitespace() {
        result.push(value.parse().unwrap());
    }

    result
}