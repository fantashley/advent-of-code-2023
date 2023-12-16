use std::{cmp::Ordering, collections::HashMap, fs, process::exit};

const CARD_RANKS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn main() {
    let contents = match fs::read_to_string("input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            exit(1);
        }
    };

    let hands: HashMap<String, u16> = contents
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let hand = iter.next().unwrap().to_string();
            let bet = iter.next().unwrap().parse::<u16>().unwrap();
            (hand, bet)
        })
        .collect();

    let mut sorted_hands: Vec<&String> = hands.keys().collect();
    sorted_hands.sort_unstable_by(|a, b| match hand_type(a).cmp(&hand_type(b)) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => second_ordering(a, b),
        Ordering::Greater => Ordering::Greater,
    });

    let num_hands = sorted_hands.len();
    let total_winnings: usize = sorted_hands
        .iter()
        .enumerate()
        .map(|hand| hands[*hand.1] as usize * (num_hands - hand.0 as usize))
        .sum();

    println!("The total winnings are: {}", total_winnings);
}

fn second_ordering(a: &String, b: &String) -> Ordering {
    let card_ranks: HashMap<&char, usize> = CARD_RANKS
        .iter()
        .enumerate()
        .map(|card| (card.1, card.0))
        .collect();

    for (a, b) in a.chars().zip(b.chars()) {
        match card_ranks[&a].cmp(&card_ranks[&b]) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => continue,
            Ordering::Greater => return Ordering::Greater,
        }
    }

    Ordering::Equal
}

fn hand_type(hand: &String) -> u8 {
    let mut counts: HashMap<char, u8> = HashMap::with_capacity(hand.len());
    for c in hand.chars() {
        counts.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut sorted_counts: Vec<&u8> = counts.values().collect();
    sorted_counts.sort_unstable();

    match sorted_counts.pop() {
        Some(5) => 1,
        Some(4) => 2,
        Some(3) => match sorted_counts.pop() {
            Some(2) => 3,
            _ => 4,
        },
        Some(2) => match sorted_counts.pop() {
            Some(2) => 5,
            _ => 6,
        },
        _ => 7,
    }
}
