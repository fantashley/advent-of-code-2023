use std::{fs, process::exit};

fn main() {
    let contents = match fs::read_to_string("input.txt") {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            exit(1);
        }
    };

    let mut lines = contents.lines();

    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect();

    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect();

    let product: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| num_possibilities(*time, *distance) as u64)
        .product();

    println!("The product is: {}", product);
}

fn num_possibilities(time: u32, distance: u32) -> u32 {
    for i in 1..time {
        if get_distance(i, time) > distance {
            return time + 1 - 2 * i;
        }
    }
    0
}

fn get_distance(hold_time: u32, race_time: u32) -> u32 {
    let speed = hold_time;
    let move_time = race_time - hold_time;
    speed * move_time
}
