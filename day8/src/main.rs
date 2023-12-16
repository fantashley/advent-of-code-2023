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

    let mut current_elements: Vec<String> = map
        .keys()
        .filter_map(|k| {
            if k.chars().last().unwrap() == 'A' {
                Some(k.clone())
            } else {
                None
            }
        })
        .collect();

    let mut steps = 0;
    for i in instructions {
        steps += 1;
        let mut all_zs = true;
        for e in current_elements.clone().iter().enumerate() {
            let (l, r) = map.get(e.1).unwrap();
            current_elements[e.0] = if i == 'L' { l.clone() } else { r.clone() };
            if current_elements[e.0].chars().last().unwrap() != 'Z' {
                all_zs = false;
            }
        }
        if all_zs {
            break;
        }
    }

    println!("Found the end in {} steps", steps);
}
