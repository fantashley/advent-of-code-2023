use std::{collections::HashSet, fs, process::exit};

fn main() {
    let contents = match fs::read_to_string("input.txt") {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            exit(1);
        }
    };

    let sum: u32 = contents
        .lines()
        .map(|line| {
            let mut numbers_only = line.split(':').last().unwrap().split('|');
            let winning_numbers: HashSet<u8> = HashSet::from_iter(
                numbers_only
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|num| num.parse().unwrap()),
            );
            let your_numbers: HashSet<u8> = HashSet::from_iter(
                numbers_only
                    .last()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|num| num.parse().unwrap()),
            );
            let num_matches = winning_numbers.intersection(&your_numbers).count() as u32;
            if num_matches == 0 {
                num_matches
            } else {
                let base: u32 = 2;
                base.pow(num_matches - 1)
            }
        })
        .sum();

    println!("The sum is: {}", sum);
}
