use std::collections::HashMap;

struct Mapping {
    to_from: Vec<(isize, isize, isize)>,
}

impl Mapping {
    fn new() -> Self {
        Mapping {
            to_from: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Range {
    start: isize,
    end: isize,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

fn find_lowest_location(seeds: Vec<isize>, mappings: HashMap<String, Mapping>) -> isize {
    let categories = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut current_number = seeds;
    let mut mapped_number = current_number.clone();
    for category in categories {
        let mapping = mappings.get(category).unwrap();
        for (to, from, range) in mapping.to_from.iter() {
            for (i, seed) in current_number.iter().enumerate() {
                if seed >= from && *seed < from + range {
                    mapped_number[i] = *to + (seed - *from)
                }
            }
        }
        current_number = mapped_number.clone();
    }
    mapped_number.into_iter().min().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut mappings: HashMap<String, Mapping> = HashMap::new();
    let mut current_category = String::new();
    let mut seeds: Vec<isize> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "seeds:" => {
                parts[1..]
                    .iter()
                    .for_each(|seed| seeds.push(seed.parse().unwrap()));
            }
            "seed-to-soil"
            | "soil-to-fertilizer"
            | "fertilizer-to-water"
            | "water-to-light"
            | "light-to-temperature"
            | "temperature-to-humidity"
            | "humidity-to-location" => {
                current_category = parts[0].to_string();
                mappings.insert(current_category.clone(), Mapping::new());
            }
            _ => {
                if let Some(mapping) = mappings.get_mut(&current_category) {
                    let tuple = (
                        parts[0].parse().unwrap(),
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                    );
                    mapping.to_from.push(tuple);
                }
            }
        }
    }

    Some(find_lowest_location(seeds, mappings) as u32)
}

fn find_lowest_location_2(seeds: Vec<Range>, mappings: HashMap<String, Mapping>) -> isize {
    let categories = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut current_number = seeds;
    let mut mapped_number = Vec::new();
    for category in categories {
        let mapping = mappings.get(category).unwrap();
        'stack: while let Some(Range { start, end }) = current_number.pop() {
            for (to, from, range) in mapping.to_from.iter() {
                let max_start = start.max(*from);
                let min_end = end.min(from + range);
                if max_start < min_end {
                    mapped_number.push(Range {
                        start: max_start - (from - to),
                        end: min_end - (from - to),
                    });
                    if start < *from {
                        current_number.push(Range {
                            start,
                            end: max_start - 1,
                        })
                    }
                    if end > from + range {
                        current_number.push(Range {
                            start: min_end,
                            end,
                        })
                    }
                    continue 'stack;
                }
            }
            mapped_number.push(Range { start, end })
        }
        current_number = mapped_number.clone();
        mapped_number = Vec::new();
    }
    current_number.into_iter().min().unwrap().start
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mappings: HashMap<String, Mapping> = HashMap::new();
    let mut current_category = String::new();
    let mut seeds: Vec<Range> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "seeds:" => {
                let range = (1..parts[1..].len()).step_by(2);
                for i in range {
                    let start = parts[i].parse().unwrap();
                    seeds.push(Range {
                        start,
                        end: parts[i + 1].parse::<isize>().unwrap() + start - 1,
                    })
                }
            }
            "seed-to-soil"
            | "soil-to-fertilizer"
            | "fertilizer-to-water"
            | "water-to-light"
            | "light-to-temperature"
            | "temperature-to-humidity"
            | "humidity-to-location" => {
                current_category = parts[0].to_string();
                mappings.insert(current_category.clone(), Mapping::new());
            }
            _ => {
                if let Some(mapping) = mappings.get_mut(&current_category) {
                    let tuple = (
                        parts[0].parse().unwrap(),
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                    );
                    mapping.to_from.push(tuple);
                }
            }
        }
    }
    Some(find_lowest_location_2(seeds, mappings) as u32)
}

aoc::solution!(5);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 5)), Some(35));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 5)), Some(46));
    }
}
