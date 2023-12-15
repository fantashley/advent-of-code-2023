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

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .map(|num| {
            let mut num_str = num.to_string();
            num_str.retain(|c| !c.is_whitespace());
            num_str.parse().unwrap()
        })
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .map(|num| {
            let mut num_str = num.to_string();
            num_str.retain(|c| !c.is_whitespace());
            num_str.parse().unwrap()
        })
        .collect();

    let product: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| num_possibilities(*time, *distance) as u64)
        .product();

    println!("The product is: {}", product);
}

fn num_possibilities(time: u64, distance: u64) -> u64 {
    for i in 1..time {
        if get_distance(i, time) > distance {
            return time + 1 - 2 * i;
        }
    }
    0
}

fn get_distance(hold_time: u64, race_time: u64) -> u64 {
    let speed = hold_time;
    let move_time = race_time - hold_time;
    speed * move_time
}
