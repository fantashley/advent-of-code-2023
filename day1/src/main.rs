use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total = 0;
    let lines = read_lines("input.txt");
    let lines = match lines {
        Err(err) => {
            println!("Could not read file: {}", err);
            return;
        }
        Ok(lines) => lines,
    };
    for line in lines {
        if let Ok(ip) = line {
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;
            let mut char_iter = ip.char_indices();
            while let Some(char) = char_iter.next() {
                let (_size, c) = char;
                if !c.is_digit(10) {
                    continue;
                }
                if first == None {
                    first = Some(c);
                }
                last = Some(c);
            }

            let combined = match (first, last) {
                (Some(first), Some(last)) => vec![first, last],
                _ => {
                    println!("No digits found");
                    continue;
                }
            };

            let number = String::from_iter(combined).parse::<u32>();
            if let Ok(number) = number {
                total += number;
            };
        }
    }

    println!("The total is: {}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
