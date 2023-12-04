use std::collections::{HashMap, HashSet};

type Point = (i32, i32);

type MyMap = HashMap<Point, HashSet<(u32, i32, i32, i32)>>;

pub fn part_one(input: &str) -> Option<u32> {
    let mut nums = Vec::new();
    let mut signs = Vec::new();
    let mut curent = String::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '.' || !char.is_ascii_digit() {
                if curent.parse::<u32>().is_ok() {
                    nums.push((
                        curent.parse::<u32>().ok().unwrap(),
                        i as i32,
                        (j - curent.len()) as i32,
                        (j - 1) as i32,
                    ));
                    curent.clear()
                };
            } else {
                curent.push(char);
            };
            if !char.is_ascii_digit() && char != '.' {
                signs.push((i as i32, j as i32))
            }
        }
        if curent.parse::<u32>().is_ok() {
            nums.push((
                curent.parse::<u32>().ok().unwrap(),
                i as i32,
                (line.len() - curent.len() - 1) as i32,
                (line.len() - 1) as i32,
            ));
            curent.clear()
        }
    }
    let steps = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut sum = 0;
    for (num, row, start, end) in nums {
        'single_num: for i in start..=end {
            for (u, d) in steps {
                if signs.contains(&((row + u), (i + d))) {
                    sum += num;
                    break 'single_num;
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nums = Vec::new();
    let mut signs = Vec::new();
    let mut curent = String::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '.' || !char.is_ascii_digit() {
                if curent.parse::<u32>().is_ok() {
                    nums.push((
                        curent.parse::<u32>().ok().unwrap(),
                        i as i32,
                        (j - curent.len()) as i32,
                        (j - 1) as i32,
                    ));
                    curent.clear()
                };
            } else {
                curent.push(char);
            };
            if char == '*' {
                signs.push((i as i32, j as i32))
            }
        }
        if curent.parse::<u32>().is_ok() {
            nums.push((
                curent.parse::<u32>().ok().unwrap(),
                i as i32,
                (line.len() - curent.len() - 1) as i32,
                (line.len() - 1) as i32,
            ));
            curent.clear()
        }
    }
    let steps = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut sum = 0;
    let mut map: MyMap = HashMap::new();
    for (num, row, start, end) in nums {
        for i in start..=end {
            for (u, d) in steps {
                if signs.contains(&((row + u), (i + d))) {
                    let key = ((row + u), (i + d));
                    let entry = map.entry(key).or_default();
                    entry.insert((num, row, start, end));
                }
            }
        }
    }
    for (_, v) in map.iter() {
        if v.len() == (2_usize) {
            let mut prod = 1;
            for val in v.iter() {
                prod *= val.0;
            }
            sum += prod;
        }
    }

    Some(sum)
}

aoc::solution!(3);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 3)),
            Some(4361)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 3)),
            Some(467835)
        );
    }
}
