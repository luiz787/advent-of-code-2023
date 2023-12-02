fn main() {
    let input = include_str!("../input");

    let result: u64 = input.lines().map(|line| {
        let first_digit = line.chars().filter(|c| c.is_digit(10)).next().unwrap();
        let last_digit = line.chars().rev().filter(|c| c.is_digit(10)).next().unwrap();

        let number: u64 = (format!("{}{}", first_digit, last_digit)).parse().unwrap();
        number
    }).sum();

    println!("{}", result)
}
