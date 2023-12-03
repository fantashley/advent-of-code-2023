use std::{collections::HashMap, fs};

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
        Ok(a) => a,
    };

    let color_maxes = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    let sum: u32 = input
        .lines()
        .map(|line| {
            let game = parse_game(line);
            match game.is_valid(&color_maxes) {
                true => game.id as u32,
                false => 0 as u32,
            }
        })
        .sum();

    println!("The sum is {}", sum);
}

struct Game {
    id: u8,
    color_maxes: HashMap<Color, u8>,
}

impl Game {
    fn is_valid(&self, color_maxes: &HashMap<Color, u8>) -> bool {
        for (color, count) in color_maxes {
            if self.color_maxes.get(color).unwrap_or(&0) > count {
                return false;
            }
        }
        true
    }
}

#[derive(Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Color {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

fn parse_game(line: &str) -> Game {
    let mut iter = line.split_ascii_whitespace();
    iter.next();
    let id = iter
        .next()
        .unwrap()
        .trim_end_matches(':')
        .parse::<u8>()
        .unwrap();
    let mut color_maxes: HashMap<Color, u8> = HashMap::new();

    while let Some(count) = iter.next() {
        let count: u8 = count.parse().unwrap();
        let color = iter.next().unwrap();
        let color = color.trim_end_matches(|c| c == ';' || c == ',');
        let max = color_maxes
            .entry(Color::try_from(color).unwrap())
            .or_insert(count);
        if count > *max {
            *max = count;
        }
    }

    Game { id, color_maxes }
}
