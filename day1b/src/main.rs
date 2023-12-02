use std::{vec, collections::HashMap};

fn main() {
    let input = include_str!("../input");

    let result: u64 = input.lines().map(|line| {
        let first_digit = find_first_digit(line);
        let last_digit = find_last_digit(line);

        let number: u64 = (format!("{}{}", first_digit, last_digit)).parse().unwrap();
        number
    }).sum();

    println!("{}", result)
}

fn find_first_digit(line: &str) -> char {
    let digit_names = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut min_index = line.len() + 1;
    let mut min_size = 0;
    for digit_name in digit_names {
        if line.contains(digit_name) {
            let digit_position = line.find(digit_name).unwrap();
            if min_index > digit_position {
                min_index = digit_position;
                min_size = digit_name.len();

            }
        }
    }

    for digit in digits {
        if line.contains(digit) {
            let digit_position = line.find(digit).unwrap();
            if min_index > digit_position {
                min_index = digit_position;
                min_size = 1;
            }
        }
    }

    if min_size == 1 {
        return line.chars().skip(min_index).take(1).next().unwrap()
    } else {
        let my_value = &line[min_index..(min_index+min_size)];

        let digits_mapping = HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);

        digits_mapping[my_value]
    }

}

fn find_last_digit(line: &str) -> char {
    let digit_names = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut max_index = -1 as i32;
    let mut max_size = 0;
    for digit_name in digit_names {
        if line.contains(digit_name) {
            let digit_position = line.match_indices(digit_name).last().unwrap().0;
            if max_index < digit_position as i32 {

                max_index = digit_position as i32;
                max_size = digit_name.len();

            }
        }
    }

    for digit in digits {
        if line.contains(digit) {
            let digit_position = line.match_indices(digit).last().unwrap().0;
            if max_index < digit_position as i32 {
                max_index = digit_position as i32;
                max_size = 1;
            }
        }
    }

    if max_size == 1 {
        return line.chars().skip(max_index as usize).take(1).next().unwrap()
    } else {
        let my_value = &line[max_index as usize..(max_index as usize + max_size)];

        let digits_mapping = HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);

        digits_mapping[my_value]
    }
}
