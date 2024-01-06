use std::{collections::HashMap, fs};

#[derive(Eq, Hash, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
    None,
}

const PIPES: [(char, (Direction, Direction)); 7] = [
    ('|', (Direction::N, Direction::S)),
    ('-', (Direction::E, Direction::W)),
    ('L', (Direction::N, Direction::E)),
    ('J', (Direction::N, Direction::W)),
    ('7', (Direction::S, Direction::W)),
    ('F', (Direction::S, Direction::E)),
    ('.', (Direction::None, Direction::None)),
];

const OPPOSITE_DIR: [(Direction, Direction); 4] = [
    (Direction::N, Direction::S),
    (Direction::S, Direction::N),
    (Direction::E, Direction::W),
    (Direction::W, Direction::E),
];

fn main() {
    let steps = find_steps(fs::read_to_string("input.txt").expect("unable to read input file"));
    println!("Step count: {}", (steps + 1) / 2);
}

fn find_steps(grid: String) -> u32 {
    let grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
    let pipes: HashMap<char, (Direction, Direction)> = PIPES.into();
    let opposite_dirs: HashMap<Direction, Direction> = OPPOSITE_DIR.into();

    let starting_point = {
        let mut x: usize = 0;
        let mut y: usize = 0;
        'outer: for i in &grid {
            x = 0;
            for j in i {
                if *j == 'S' {
                    break 'outer;
                }
                x += 1;
            }
            y += 1;
        }
        (y, x)
    };

    let (mut y, mut x) = starting_point;
    let starting_direction = starting_direction(&pipes, &grid, starting_point);
    (x, y) = get_coordinates(x, y, &starting_direction);
    let mut prev_direction = &opposite_dirs[&starting_direction];
    let mut steps: u32 = 1;
    loop {
        if (y, x) == starting_point {
            break;
        }

        steps += 1;

        let (dir1, dir2) = &pipes[&grid[y][x]];
        let next = if prev_direction == dir1 { dir2 } else { dir1 };

        (x, y) = get_coordinates(x, y, next);

        prev_direction = &opposite_dirs[&next]
    }

    steps
}

fn starting_direction(
    pipes: &HashMap<char, (Direction, Direction)>,
    grid: &Vec<Vec<char>>,
    starting_point: (usize, usize),
) -> Direction {
    let (y, x) = starting_point;
    if let Some(north_row) = grid.get(y - 1) {
        let pipe = north_row[x];
        let (dir1, dir2) = &pipes[&pipe];
        if *dir1 == Direction::S || *dir2 == Direction::S {
            return Direction::N;
        }
    }
    if let Some(east_pipe) = grid[y].get(x + 1) {
        let (dir1, dir2) = &pipes[east_pipe];
        if *dir1 == Direction::W || *dir2 == Direction::W {
            return Direction::E;
        }
    }
    if let Some(south_row) = grid.get(y + 1) {
        let pipe = south_row[x];
        let (dir1, dir2) = &pipes[&pipe];
        if *dir1 == Direction::N || *dir2 == Direction::N {
            return Direction::S;
        }
    }
    if let Some(west_pipe) = grid[y].get(x - 1) {
        let (dir1, dir2) = &pipes[west_pipe];
        if *dir1 == Direction::E || *dir2 == Direction::E {
            return Direction::W;
        }
    }

    Direction::None
}

fn get_coordinates(x: usize, y: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::N => (x, y - 1),
        Direction::S => (x, y + 1),
        Direction::E => (x + 1, y),
        Direction::W => (x - 1, y),
        Direction::None => (0, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_steps() {
        let steps = find_steps(fs::read_to_string("test_input.txt").unwrap());
        assert_eq!(steps, 16);
    }
}
