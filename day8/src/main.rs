use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("error reading input file");
    let mut lines = contents.lines();
    let instructions = lines.next().unwrap();

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

    let starting_elements: Vec<&String> = map
        .keys()
        .filter(|node| node.chars().last().unwrap() == 'A')
        .collect();

    let mut all_steps: Vec<u32> = Vec::with_capacity(starting_elements.len());

    for i in starting_elements {
        let mut current_element = i;
        let mut steps = 0;
        for i in instructions.chars().cycle() {
            let (l, r) = map.get(current_element).unwrap();
            current_element = if i == 'L' { l } else { r };
            steps += 1;
            if current_element.chars().last().unwrap() == 'Z' {
                all_steps.push(steps);
                break;
            }
        }
    }

    let mut lcm: u64 = all_steps.pop().unwrap() as u64;
    for step_count in all_steps {
        lcm = num::integer::lcm(lcm, step_count as u64);
    }

    println!("The number of steps is: {}", lcm);
}
