use std::collections::{HashMap, HashSet, VecDeque};
use std::vec;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let input = include_str!("../input");

    let mut maze: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let (si, sj) = find_s_position(&maze);
    let s_value = infer_s_value(&maze, si, sj);

    maze[si][sj] = s_value;

    let neighbors = get_neighbors(&maze, si, sj);
    let mut stack: VecDeque<(usize, usize, Direction)> = VecDeque::new();

    stack.push_back(neighbors[0]);

    let mut steps = 1;
    let mut seen = HashSet::new();
    seen.insert((si, sj));

    while !stack.is_empty() {
        let (i, j, _dir) = stack.pop_back().unwrap();

        seen.insert((i, j));
        let curr_neighbors = get_neighbors(&maze, i, j);

        for (nx, ny, ndir) in curr_neighbors.clone() {
            let is_not_starting_position = nx != si || ny != sj;

            if is_not_starting_position && !seen.contains(&(nx, ny)) {
                stack.push_back((nx, ny, ndir));
            }
        }

        steps += 1;
    }

    println!("{}", steps / 2);
}

fn infer_s_value(maze: &Vec<Vec<char>>, si: usize, sj: usize) -> char {
    let neighbors = get_neighbors(maze, si, sj);

    let mut neighbors_by_position = HashMap::new();
    for (nx, ny, dir) in neighbors {
        neighbors_by_position.insert(dir, maze[nx][ny]);
    }

    let up_neighbor = *neighbors_by_position.get(&Direction::UP).unwrap();
    if up_neighbor != '|' && up_neighbor != 'F' && up_neighbor != '7' {
        neighbors_by_position.remove(&Direction::UP);
    }

    let down_neighbor = *neighbors_by_position.get(&Direction::DOWN).unwrap();
    if down_neighbor != '|' && down_neighbor != 'J' && down_neighbor != 'L' {
        neighbors_by_position.remove(&Direction::DOWN);
    }

    let left_neighbor = *neighbors_by_position.get(&Direction::LEFT).unwrap();
    if left_neighbor != '-' && left_neighbor != 'F' && left_neighbor != 'L' {
        neighbors_by_position.remove(&Direction::LEFT);
    }

    let right_neighbor = *neighbors_by_position.get(&Direction::RIGHT).unwrap();
    if right_neighbor != '-' && right_neighbor != 'J' && right_neighbor != '7' {
        neighbors_by_position.remove(&Direction::RIGHT);
    }

    let mut remaining: Vec<_> = neighbors_by_position.keys().map(|d| d.clone()).collect();

    remaining.sort();

    if remaining == vec![Direction::UP, Direction::DOWN] {
        '|'
    } else if remaining == vec![Direction::UP, Direction::LEFT] {
        'J'
    } else if remaining == vec![Direction::UP, Direction::RIGHT] {
        'L'
    } else if remaining == vec![Direction::LEFT, Direction::RIGHT] {
        '-'
    } else if remaining == vec![Direction::DOWN, Direction::LEFT] {
        '7'
    } else {
        'F'
    }
}

fn find_s_position(maze: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, line) in maze.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == 'S' {
                return (i, j);
            }
        }
    }

    unreachable!("S has to exist");
}

fn get_neighbors(maze: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<(usize, usize, Direction)> {
    let deltas = match maze[i][j] {
        '|' => vec![(-1, 0, Direction::UP), (1, 0, Direction::DOWN)],
        'L' => vec![(-1, 0, Direction::UP), (0, 1, Direction::RIGHT)],
        'J' => vec![(0, -1, Direction::LEFT), (-1, 0, Direction::UP)],
        '7' => vec![(0, -1, Direction::LEFT), (1, 0, Direction::DOWN)],
        'F' => vec![(1, 0, Direction::DOWN), (0, 1, Direction::RIGHT)],
        '-' => vec![(0, -1, Direction::LEFT), (0, 1, Direction::RIGHT)],
        'S' => vec![
            (-1, 0, Direction::UP),
            (1, 0, Direction::DOWN),
            (0, -1, Direction::LEFT),
            (0, 1, Direction::RIGHT),
        ],
        _ => unreachable!(),
    };

    deltas
        .iter()
        .map(|(dx, dy, dir)| (dx + i as i32, dy + j as i32, dir))
        .map(|(x, y, dir)| (x as usize, y as usize, *dir))
        .collect()
}
