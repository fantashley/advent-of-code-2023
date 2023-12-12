use std::{collections::HashSet, fs, process::exit};

fn main() {
    let contents = match fs::read_to_string("input.txt") {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            exit(1);
        }
    };

    let (sum, matches) = get_matches(&contents);

    println!("The sum is: {}", sum);
    println!(
        "The total number of scratchcards is: {}",
        add_matches(&matches)
    )
}

fn get_matches(input: &str) -> (u32, Vec<u32>) {
    let mut matches: Vec<u32> = Vec::new();

    let sum: u32 = input
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
            matches.push(num_matches);
            if num_matches == 0 {
                num_matches
            } else {
                let base: u32 = 2;
                base.pow(num_matches - 1)
            }
        })
        .sum();

    (sum, matches)
}

fn add_matches(matches: &Vec<u32>) -> u32 {
    let mut totals: Vec<u32> = vec![1; matches.len()];
    for i in (0..matches.len()).rev() {
        match matches[i] {
            0 => totals[i] = 1,
            n => {
                for j in i + 1..i + 1 + n as usize {
                    totals[i] += totals[j];
                }
            }
        }
    }
    totals.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{add_matches, get_matches};
    #[test]
    fn test_add_matches() {
        let contents = match fs::read_to_string("testdata/test_input.txt") {
            Ok(c) => c,
            Err(err) => {
                panic!("Error reading file: {}", err);
            }
        };

        let (sum, matches) = get_matches(&contents);
        assert_eq!(sum, 13);
        assert_eq!(add_matches(&matches), 30);
    }
}
