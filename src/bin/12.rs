// use std::collections::{hash_map, HashMap, HashSet};

// use itertools::Itertools;

// fn replace_char_at_index(input: &str, index: usize, new_char: char) -> String {
//     let mut chars: Vec<char> = input.chars().collect();
//     if let Some(existing_char) = chars.get_mut(index) {
//         *existing_char = new_char;
//     }
//     chars.into_iter().collect()
// }

// fn correct(seq: &str, ins: &Vec<usize>) -> bool {
//     if ins.iter().sum::<usize>() < seq.chars().filter(|&c| c == '#').count() {
//         return false;
//     }
//     let result: Vec<&str> = seq.split(|c| c != '#').filter(|&s| !s.is_empty()).collect();
//     if result.len() < ins.len() {
//         return false;
//     }
//     for (seq, num) in result.iter().zip(ins) {
//         if seq.len() != *num {
//             return false;
//         }
//     }
//     true
// }

// fn solve(seq: &str, ins: Vec<usize>) -> usize {
//     let mut valid = HashSet::new();
//     let mut count = 0;
//     // let mut hash_map = HashMap::new();

//     fn rec_solve(
//         seq: &str,
//         ins: &Vec<usize>,
//         count: &mut u32,
//         valid: &mut HashSet<String>,
//     ) -> bool {
//         if ins.iter().sum::<usize>() < seq.chars().filter(|&c| c == '#').count() {
//             return false;
//         } else if seq.find('?').is_some() {
//             let index = seq.find('?').unwrap();
//             let new_hash = replace_char_at_index(seq, index, '#');
//             let new_dot = replace_char_at_index(seq, index, '.');
//             let a = rec_solve(&new_hash, ins, count, valid);
//             let b = rec_solve(&new_dot, ins, count, valid);
//             return a | b;
//         } else {
//             if correct(seq, ins) {
//                 valid.insert(seq.to_string());
//                 *count += 1;
//                 return true;
//             }
//             false
//         }
//     }

//     rec_solve(seq, &ins, &mut count, &mut valid);
//     // valid.len()
//     count as usize
// }

use std::collections::{hash_map, HashMap, HashSet};

use itertools::Itertools;

#[derive(PartialEq, PartialOrd, Hash)]
enum ThreeValue {
    Can,
    Must,
    MustNot,
}

fn replace_char_at_index(input: &str, index: usize, new_char: char) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    if let Some(existing_char) = chars.get_mut(index) {
        *existing_char = new_char;
    }
    chars[index..].into_iter().collect()
}

fn solve(seq: &str, ins: Vec<usize>) -> usize {
    let mut count = 0;
    let mut memo_hash: HashMap<(ThreeValue, &str, &Vec<usize>), usize> = HashMap::new();

    fn rec_solve(
        start: ThreeValue,
        seq: &str,
        ins: &Vec<usize>,
        memo_hash: &mut HashMap<(ThreeValue, &str, &Vec<usize>), usize>,
    ) -> usize {
        match seq.chars().next() {
            None => {
                if !ins.is_empty() {
                    0
                } else {
                    1
                }
            }
            Some(ch) => match ch {
                '.' => match start {
                    ThreeValue::Must => 0,
                    _ => {
                        if !memo_hash.contains_key(&(
                            ThreeValue::Can,
                            seq[1..].to_string(),
                            ins.to_vec(),
                        )) {
                            let value = rec_solve(ThreeValue::Can, &seq[1..], ins, memo_hash);
                            memo_hash.insert(
                                (ThreeValue::Can, seq[1..].to_string(), ins.to_vec()),
                                value,
                            );
                        }
                        *memo_hash
                            .get(&(ThreeValue::Can, seq[1..].to_string(), ins.to_vec()))
                            .unwrap()
                    }
                },
                '#' => match start {
                    ThreeValue::MustNot => 0,
                    _ => {
                        if ins.len() == 0 {
                            return 0;
                        } else if ins[0] == 1 {
                            rec_solve(
                                ThreeValue::MustNot,
                                &seq[1..],
                                &ins[1..].to_vec(),
                                memo_hash,
                            )
                        } else {
                            let mut ins_clone = ins.clone();
                            if let Some(first) = ins_clone.first_mut() {
                                *first -= 1;
                            }
                            rec_solve(ThreeValue::Must, &seq[1..], &ins_clone, memo_hash)
                        }
                    }
                },
                '?' => match start {
                    ThreeValue::Must => {
                        if ins[0] > 1 {
                            rec_solve(
                                ThreeValue::Must,
                                &replace_char_at_index(seq, 0, '#'),
                                &ins,
                                memo_hash,
                            )
                        } else {
                            rec_solve(
                                ThreeValue::MustNot,
                                &seq[1..],
                                &ins[1..].to_vec(),
                                memo_hash,
                            )
                        }
                    }
                    ThreeValue::MustNot => rec_solve(ThreeValue::Can, &seq[1..], ins, memo_hash),
                    ThreeValue::Can => {
                        if ins.is_empty() {
                            return rec_solve(ThreeValue::MustNot, &seq[1..], ins, memo_hash);
                        }
                        let a = if ins[0] > 1 {
                            rec_solve(
                                ThreeValue::Must,
                                &replace_char_at_index(seq, 0, '#'),
                                &ins,
                                memo_hash,
                            )
                        } else {
                            rec_solve(
                                ThreeValue::MustNot,
                                &seq[1..],
                                &ins[1..].to_vec(),
                                memo_hash,
                            )
                        };
                        let b = rec_solve(ThreeValue::Can, &seq[1..], ins, memo_hash);
                        a + b
                    }
                },
                _ => unreachable!(),
            },
        }
    }

    rec_solve(ThreeValue::Can, seq, &ins, &mut memo_hash)
    // count as usize
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (seq, ins) = line.split_once(' ').unwrap();
        let ins = ins
            .split(',')
            .map(|c| c.parse::<usize>().ok().unwrap())
            .collect_vec();
        sum += solve(seq, ins)
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    return None;
    let mut sum = 0;
    for line in input.lines() {
        let (seq, ins) = line.split_once(' ').unwrap();
        let ins = ins
            .repeat(5)
            .split(',')
            .map(|c| c.parse::<usize>().ok().unwrap())
            .collect_vec();
        sum += solve(seq.repeat(5).as_str(), ins)
    }
    Some(sum as u32)
}

aoc::solution!(12);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 12)),
            Some(21)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 12)),
            Some(525152)
        );
    }
}
