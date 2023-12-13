use std::{cmp::Ordering, collections::BinaryHeap, fs, process::exit};

fn main() {
    let contents = match fs::read_to_string("input.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            exit(1);
        }
    };

    let mut lines = contents.lines();

    // Seeds line
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect();

    let mut mapping_builder: Vec<MappingBuilder> = Vec::new();
    for line in lines {
        if line.contains("map") {
            let current_mapping = MappingBuilder::new();
            mapping_builder.push(current_mapping);
            continue;
        }

        if let Some(range) = Range::from(
            line.split_ascii_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect(),
        ) {
            match mapping_builder.last_mut() {
                Some(map) => map.add_range(range),
                None => continue,
            }
        }
    }

    let mappings: Vec<Mapping> = mapping_builder.into_iter().map(|m| m.mapping()).collect();
    let mut min_location: Option<u64> = None;
    for seed in seeds {
        let mut src = seed;
        for mapping in &mappings {
            src = mapping.get_mapping(src)
        }
        min_location = match min_location {
            Some(l) => {
                if src < l {
                    Some(src)
                } else {
                    Some(l)
                }
            }
            None => Some(src),
        }
    }

    match min_location {
        Some(l) => println!("The minimum location is: {}", l),
        None => eprintln!("No minimum location found"),
    }
}

#[derive(Eq)]
struct Range {
    dest: u64,
    src: u64,
    length: u64,
}

impl Range {
    fn from(nums: Vec<u64>) -> Option<Range> {
        match (nums.get(0), nums.get(1), nums.get(2)) {
            (Some(a), Some(b), Some(c)) => Some(Range {
                dest: *a,
                src: *b,
                length: *c,
            }),
            _ => None,
        }
    }

    fn get_mapping(&self, number: u64) -> Option<u64> {
        match (number < self.src, number >= self.src + self.length) {
            (false, false) => Some(self.dest + number - self.src),
            _ => None,
        }
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.src.cmp(&other.src)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src
    }
}

struct MappingBuilder {
    ranges: BinaryHeap<Range>,
}

impl MappingBuilder {
    fn new() -> MappingBuilder {
        MappingBuilder {
            ranges: BinaryHeap::new(),
        }
    }

    fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }

    fn mapping(self) -> Mapping {
        let sorted_ranges = self.ranges.into_sorted_vec();
        Mapping {
            sorted_ranges: sorted_ranges,
        }
    }
}

struct Mapping {
    sorted_ranges: Vec<Range>,
}

impl Mapping {
    fn get_mapping(&self, number: u64) -> u64 {
        let range_index = self.sorted_ranges.partition_point(|r| r.src <= number);
        match self.sorted_ranges.get(range_index - 1) {
            Some(range) => match range.get_mapping(number) {
                Some(m) => m,
                None => number,
            },
            None => number,
        }
    }
}
