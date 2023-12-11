use std::{fs, process::exit};

fn main() {
    let contents = match fs::read_to_string("input.txt") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            exit(1);
        }
    };

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        grid.push(line.chars().collect());
    }

    match evaluate_numbers(&grid) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(err) => {
            eprintln!("Error encountered: {:?}", err);
        }
    }
}

fn evaluate_numbers(grid: &Vec<Vec<char>>) -> Result<u32, Error> {
    let max_x = match grid.get(0) {
        Some(row) => row.len(),
        None => return Err(Error::EmptyGrid),
    };
    let max_y = grid.len();
    let mut sum: u32 = 0;

    for y in 0..max_y {
        let mut number: Vec<u8> = Vec::new();
        let mut is_adjacent = false;
        for x in 0..max_x {
            let c = grid[y][x];
            if !c.is_digit(10) {
                continue;
            }

            number.push(c.to_digit(10).unwrap() as u8);
            is_adjacent = if is_adjacent {
                is_adjacent
            } else {
                adjacent_to_symbol(grid, y, x)?
            };

            if let Some(next) = grid[y].get(x + 1) {
                if next.is_digit(10) {
                    continue;
                }
            }

            // Reached last digit in number
            if is_adjacent {
                let mut value: u32 = 0;
                for (i, num) in number.iter().rev().enumerate() {
                    value += *num as u32 * 10_u32.pow(i.try_into().unwrap());
                }
                sum += value;
            }

            // Reset tracking fields
            number.clear();
            is_adjacent = false;
        }
    }

    Ok(sum)
}

#[derive(Debug)]
enum Error {
    OutOfBounds(String),
    EmptyGrid,
}

fn adjacent_to_symbol(grid: &Vec<Vec<char>>, y: usize, x: usize) -> Result<bool, Error> {
    if y >= grid.len() {
        return Err(Error::OutOfBounds(
            "row number is out of bounds".to_string(),
        ));
    };
    if x >= grid[0].len() {
        return Err(Error::OutOfBounds(
            "column number is out of bounds".to_string(),
        ));
    }

    let x_start_index = if x == 0 { 0 } else { x - 1 };
    let y_start_index = if y == 0 { 0 } else { y - 1 };

    for row_index in y_start_index..=y + 1 {
        let row = match grid.get(row_index) {
            Some(r) => r,
            None => continue,
        };
        for col_index in x_start_index..=x + 1 {
            let val = match row.get(col_index) {
                Some(c) => c,
                None => continue,
            };
            if !val.is_digit(10) && *val != '.' {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
