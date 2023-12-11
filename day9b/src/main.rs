fn main() {
    let input = include_str!("../input");

    let input: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .rev()
                .collect()
        })
        .collect();

    let result: i64 = input.iter().map(|values| predict_next(values)).sum();

    println!("{}", result);
}

fn predict_next(values: &Vec<i64>) -> i64 {
    let mut all_zero = false;
    let mut current_values = values.clone();
    let mut rows: Vec<Vec<i64>> = vec![values.to_vec()];
    while !all_zero {
        let new_values: Vec<_> = current_values
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        if new_values.iter().all(|v| *v == 0) {
            all_zero = true;
        }

        rows.push(new_values.clone());
        current_values = new_values;
    }

    current_values.push(0);
    let mut current_sum_value = 0;
    for i in (0..rows.len() - 1).rev() {
        current_sum_value = rows[i][rows[i].len() - 1] + current_sum_value;
        rows[i].push(current_sum_value);
    }

    current_sum_value
}

