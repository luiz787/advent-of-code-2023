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

    let symbols_positions = get_symbols_positions(input);
    let numbers_info = get_numbers_info(input);

    let result: usize = numbers_info
        .iter()
        .filter(|&number_info| {
            let close_symbols: Vec<_> = symbols_positions
                .iter()
                .filter(|(x, y)| symbol_is_close_to_number(x, y, number_info))
                .collect();

            println!(
                "Symbols close to {}: {:?}",
                number_info.value, close_symbols
            );

            !close_symbols.is_empty()
        })
        .map(|number_info| number_info.value)
        .sum();

    println!("{}", result);
}

fn symbol_is_close_to_number(sym_x: &usize, sym_y: &usize, number_info: &NumberInfo) -> bool {
    sym_x.abs_diff(number_info.line) < 2
        && (sym_y.abs_diff(number_info.start_position) < 2
            || sym_y.abs_diff(number_info.end_position) < 2
            || sym_y > &number_info.start_position && sym_y < &number_info.end_position)
}

fn get_symbols_positions(input: &str) -> Vec<(usize, usize)> {
    let mut symbols_positions = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.char_indices() {
            if !ch.is_numeric() && ch != '.' {
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
