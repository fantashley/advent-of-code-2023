use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("error reading input file");
    let sum: i64 = contents
        .lines()
        .map(|line| {
            let mut nums: Vec<i64> = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            nums.reverse();
            get_next_num(&nums)
        })
        .sum();

    println!("The sum is: {}", sum);
}

fn get_next_num(nums: &Vec<i64>) -> i64 {
    let mut differences: Vec<i64> = Vec::with_capacity(nums.len() - 1);
    let mut all_equal = true;
    let mut last_num: Option<i64> = None;
    for i in 1..nums.len() {
        if let Some(n) = last_num {
            if nums[i] != n {
                all_equal = false;
            }
        } else {
            last_num = Some(nums[i]);
        }
        differences.push(nums[i] - nums[i - 1]);
    }
    if all_equal {
        return nums.last().unwrap() + differences.last().unwrap().to_owned();
    }
    nums.last().unwrap() + get_next_num(&differences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_num() {
        let tests = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
        let answers: [i64; 3] = [18, 28, 68];
        for t in tests.into_iter().enumerate() {
            let nums: Vec<i64> =
                t.1.split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect();
            assert_eq!(get_next_num(&nums), answers[t.0])
        }
    }
}
