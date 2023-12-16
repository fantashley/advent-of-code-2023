use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("error reading input file");
    let mut lines = contents.lines();
    let instructions = lines.next().unwrap().chars().cycle();

    let map: HashMap<String, (String, String)> = lines
        .skip(1)
        .map(|line| {
            let mut str = line.to_string();
            str.retain(|c| c.is_alphabetic());
            (
                (&str[0..3]).to_string(),
                ((&str[3..6]).to_string(), (&str[6..]).to_string()),
            )
        })
        .collect();

    let mut current_element = "AAA".to_string();
    let mut steps = 0;
    for i in instructions {
        let (l, r) = map.get(&current_element).unwrap();
        current_element = if i == 'L' { l.clone() } else { r.clone() };
        steps += 1;
        if current_element == "ZZZ" {
            break;
        }
    }

    println!("Found the end in {} steps", steps);
}
