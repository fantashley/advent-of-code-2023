use std::{fs, io};

pub fn star_two(path: &str) -> io::Result<u32> {
    let sum = fs::read_to_string(path)?
        .lines()
        .filter_map(|line| {
            let mut first_num: Option<u8> = None;
            let mut first_index = line.len();
            let mut last_num: Option<u8> = None;
            let mut last_index = 0;
            let (first_word, last_word) = first_last_word_indices(line);
            if let Some(num) = first_word {
                if num.0 < first_index {
                    first_index = num.0;
                    first_num = Some(num.1);
                }
            }
            if let Some(num) = last_word {
                if num.0 > last_index {
                    last_index = num.0;
                    last_num = Some(num.1);
                }
            }

            let mut digits = line.chars().enumerate().filter(|c| c.1.is_digit(10));
            let first_digit = digits.next();
            let last_digit = digits.last();
            if let Some(num) = first_digit {
                if num.0 < first_index {
                    first_index = num.0;
                    first_num = Some(num.1.to_digit(10).unwrap() as u8);
                }
                if num.0 >= last_index {
                    last_index = num.0;
                    last_num = Some(num.1.to_digit(10).unwrap() as u8);
                }
            }
            if let Some(num) = last_digit {
                if num.0 >= last_index {
                    last_index = num.0;
                    last_num = Some(num.1.to_digit(10).unwrap() as u8);
                }
            }
            match (first_num, last_num) {
                (Some(first), Some(last)) => Some(first as u32 * 10 + last as u32),
                _ => None,
            }
        })
        .sum();
    Ok(sum)
}

const NUMBER_WORDS: [(u8, &str); 10] = [
    (0, "zero"),
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn first_last_word_indices(line: &str) -> (Option<(usize, u8)>, Option<(usize, u8)>) {
    let mut first: Option<u8> = None;
    let mut first_index = line.len();
    let mut last: Option<u8> = None;
    let mut last_index = 0;
    for number in NUMBER_WORDS {
        let index = line.find(number.1);
        match index {
            None => continue,
            Some(index) => {
                if index < first_index {
                    first_index = index;
                    first = Some(number.0);
                }
            }
        }
        if let Some(index) = line.rfind(number.1) {
            if index >= last_index {
                last_index = index;
                last = Some(number.0)
            }
        }
    }

    let first = match first {
        Some(num) => Some((first_index, num)),
        None => None,
    };
    let last = match last {
        Some(num) => Some((last_index, num)),
        None => None,
    };
    (first, last)
}

#[cfg(test)]
mod tests {
    use crate::star_two;

    #[test]
    fn it_works() {
        struct Test<'a> {
            line: &'a str,
            first: Option<(usize, u8)>,
            last: Option<(usize, u8)>,
        }
        let word_tests = [
            Test {
                line: "eight9fhstbssrplmdlncmmqqnklb39ninejz",
                first: Some((0, 8)),
                last: Some((31, 9)),
            },
            Test {
                line: "5qp",
                first: None,
                last: None,
            },
            Test {
                line: "fiveqp",
                first: Some((0, 5)),
                last: Some((0, 5)),
            },
        ];
        for test in word_tests {
            assert_eq!(
                star_two::first_last_word_indices(test.line),
                (test.first, test.last)
            )
        }
    }
}
