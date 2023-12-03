use regex::Regex;

#[derive(Debug)]
struct NumberInfo {
    line: usize,
    start_position: usize,
    end_position: usize,
    value: usize,
}

fn main() {
    let input = include_str!("../input");

    let gears_positions = get_gears_positions(input);
    let numbers_info = get_numbers_info(input);

    let result: usize = gears_positions
        .iter()
        .filter_map(|(x, y)| {
            let close_numbers: Vec<_> = numbers_info
                .iter()
                .filter(|&number_info| symbol_is_close_to_number(x, y, number_info))
                .collect();

            if close_numbers.len() != 2 {
                return Option::None
            } else {
                return Option::Some(close_numbers[0].value * close_numbers[1].value)
            }
        })
        .sum();

    println!("{}", result);
}

fn symbol_is_close_to_number(sym_x: &usize, sym_y: &usize, number_info: &NumberInfo) -> bool {
    sym_x.abs_diff(number_info.line) < 2
        && (sym_y.abs_diff(number_info.start_position) < 2
            || sym_y.abs_diff(number_info.end_position) < 2
            || sym_y > &number_info.start_position && sym_y < &number_info.end_position)
}

fn get_gears_positions(input: &str) -> Vec<(usize, usize)> {
    let mut symbols_positions = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.char_indices() {
            if ch == '*' {
                symbols_positions.push((i, j));
            }
        }
    }

    symbols_positions
}

fn get_numbers_info(input: &str) -> Vec<NumberInfo> {
    let number_pattern = Regex::new(r"[0-9]+").unwrap();
    let mut numbers_info = Vec::new();

    for (i, line) in input.lines().enumerate() {
        for captures in number_pattern.captures_iter(line) {
            for capture in captures.iter().filter(|c| c.is_some()).map(|c| c.unwrap()) {
                numbers_info.push(NumberInfo {
                    line: i,
                    start_position: capture.start(),
                    end_position: capture.end() - 1,
                    value: capture.as_str().parse().unwrap(),
                });
            }
        }
    }

    numbers_info
}
