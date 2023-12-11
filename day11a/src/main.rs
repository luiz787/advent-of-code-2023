use itertools::Itertools;


fn main() {
    let input = include_str!("../input");
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let galaxies_positions = find_galaxies_positions(&grid);
    
    let empty_rows: Vec<_> = grid
        .iter()
        .enumerate()
        .filter(|(_i, row)| !row.contains(&'#'))
        .map(|(i, _row)| i)
        .collect();
    
    let mut empty_cols = Vec::new();
    let galaxies_cols: Vec<_> = galaxies_positions
        .iter()
        .map(|(_i, j)| j)
        .collect();
    for i in 0..grid[0].len() {
        if !galaxies_cols.contains(&&i) {
            empty_cols.push(i);
        }
    }

    let result: usize = galaxies_positions
        .iter()
        .combinations(2)
        .map(|pair| {
            let (i1, j1) = pair[0];
            let (i2, j2) = pair[1];



            let row_increment = empty_rows
                .iter()
                .filter(|r| *r > i1.min(i2) && *r < i1.max(i2))
                .count();

            let col_increment = empty_cols
                .iter()
                .filter(|c| *c > j1.min(j2) && *c < j1.max(j2))
                .count();

            i1.abs_diff(*i2) + j1.abs_diff(*j2) + row_increment + col_increment
        })
        .sum();

    println!("{}", result);
}

fn find_galaxies_positions(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == '#' {
                result.push((i,j));
            }
        }
    }

    result
}