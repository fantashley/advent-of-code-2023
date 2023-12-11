use std::{
    collections::{HashMap, HashSet},
    fs,
    process::exit,
};

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
        Ok((sum, ratio_sum)) => {
            println!("The sum is: {}", sum);
            println!("The ratio sum is: {}", ratio_sum);
        }
        Err(err) => {
            eprintln!("Error encountered: {:?}", err);
        }
    }
}

fn evaluate_numbers(grid: &Vec<Vec<char>>) -> Result<(u32, u32), Error> {
    let max_x = match grid.get(0) {
        Some(row) => row.len(),
        None => return Err(Error::EmptyGrid),
    };
    let max_y = grid.len();
    let mut sum: u32 = 0;
    let mut gear_nums: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for y in 0..max_y {
        let mut number: Vec<u8> = Vec::new();
        let mut is_adjacent = false;
        let mut gears: HashSet<(usize, usize)> = HashSet::new();

        for x in 0..max_x {
            let c = grid[y][x];
            if !c.is_digit(10) {
                continue;
            }

            number.push(c.to_digit(10).unwrap() as u8);
            let symbols = adjacent_to_symbol(grid, y, x)?;

            is_adjacent = if is_adjacent {
                is_adjacent
            } else {
                symbols.adjacent
            };

            gears.extend(&symbols.gears);

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

                for gear in &gears {
                    gear_nums.entry(*gear).or_insert_with(Vec::new).push(value);
                }
            }

            // Reset tracking fields
            number.clear();
            is_adjacent = false;
            gears.clear();
        }
    }

    let mut ratio_sum = 0;
    for (_gear, nums) in gear_nums {
        if nums.len() != 2 {
            continue;
        }
        ratio_sum += nums[0] * nums[1];
    }

    Ok((sum, ratio_sum))
}

#[derive(Debug)]
enum Error {
    OutOfBounds(String),
    EmptyGrid,
}

struct SymbolResult {
    adjacent: bool,
    gears: HashSet<(usize, usize)>, // y, x
}

fn adjacent_to_symbol(grid: &Vec<Vec<char>>, y: usize, x: usize) -> Result<SymbolResult, Error> {
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
    let mut gear_set: HashSet<(usize, usize)> = HashSet::new();
    let mut adjacent: bool = false;

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
                adjacent = true;
            }
            if *val == '*' {
                gear_set.insert((row_index, col_index));
            }
        }
    }

    Ok(SymbolResult {
        adjacent: adjacent,
        gears: gear_set,
    })
}
