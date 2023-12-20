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

use std::{collections::HashMap, iter::once};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Hash)]
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
    chars.into_iter().collect()
}

fn solve(seq: &str, ins: Vec<usize>) -> usize {
    let mut memo_hash: HashMap<(ThreeValue, String, Vec<usize>), usize> = HashMap::new();

    fn rec_solve(
        start: ThreeValue,
        seq: String,
        ins: &[usize],
        memo_hash: &mut HashMap<(ThreeValue, String, Vec<usize>), usize>,
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
                            seq.clone()[1..].to_string(),
                            ins.to_vec(),
                        )) {
                            let value = rec_solve(
                                ThreeValue::Can,
                                seq.clone()[1..].to_string(),
                                ins,
                                memo_hash,
                            );
                            memo_hash.insert(
                                (ThreeValue::Can, seq.clone()[1..].to_string(), ins.to_vec()),
                                value,
                            );
                        }
                        *memo_hash
                            .get(&(ThreeValue::Can, seq.clone()[1..].to_string(), ins.to_vec()))
                            .unwrap()
                    }
                },
                '#' => match start {
                    ThreeValue::MustNot => 0,
                    _ => {
                        if ins.is_empty() {
                            return 0;
                        } else if ins[0] == 1 {
                            {
                                if !memo_hash.contains_key(&(
                                    ThreeValue::MustNot,
                                    seq.clone()[1..].to_string(),
                                    ins[1..].to_vec(),
                                )) {
                                    let value = rec_solve(
                                        ThreeValue::MustNot,
                                        seq.clone()[1..].to_string(),
                                        &ins[1..],
                                        memo_hash,
                                    );
                                    memo_hash.insert(
                                        (
                                            ThreeValue::MustNot,
                                            seq.clone()[1..].to_string(),
                                            ins[1..].to_vec(),
                                        ),
                                        value,
                                    );
                                }
                                *memo_hash
                                    .get(&(
                                        ThreeValue::MustNot,
                                        seq.clone()[1..].to_string(),
                                        ins[1..].to_vec(),
                                    ))
                                    .unwrap()
                            }
                        } else {
                            let mut ins_clone = ins.to_vec();
                            if let Some(first) = ins_clone.first_mut() {
                                *first -= 1;
                            }
                            if !memo_hash.contains_key(&(
                                ThreeValue::Must,
                                seq.clone()[1..].to_string(),
                                ins_clone.to_vec(),
                            )) {
                                let value = rec_solve(
                                    ThreeValue::Must,
                                    seq.clone()[1..].to_string(),
                                    &ins_clone,
                                    memo_hash,
                                );
                                memo_hash.insert(
                                    (
                                        ThreeValue::Must,
                                        seq.clone()[1..].to_string(),
                                        ins_clone.to_vec(),
                                    ),
                                    value,
                                );
                            }
                            *memo_hash
                                .get(&(
                                    ThreeValue::Must,
                                    seq.clone()[1..].to_string(),
                                    ins_clone.to_vec(),
                                ))
                                .unwrap()
                        }
                    }
                },
                '?' => match start {
                    ThreeValue::Must => {
                        if ins[0] > 1 {
                            if !memo_hash.contains_key(&(
                                ThreeValue::Must,
                                replace_char_at_index(&seq, 0, '#'),
                                ins.to_vec(),
                            )) {
                                let value = rec_solve(
                                    ThreeValue::Must,
                                    replace_char_at_index(&seq, 0, '#'),
                                    &ins,
                                    memo_hash,
                                );
                                memo_hash.insert(
                                    (
                                        ThreeValue::Must,
                                        replace_char_at_index(&seq, 0, '#'),
                                        ins.to_vec(),
                                    ),
                                    value,
                                );
                            }
                            *memo_hash
                                .get(&(
                                    ThreeValue::Must,
                                    replace_char_at_index(&seq, 0, '#'),
                                    ins.to_vec(),
                                ))
                                .unwrap()
                        } else {
                            if !memo_hash.contains_key(&(
                                ThreeValue::MustNot,
                                seq.clone()[1..].to_string(),
                                ins[1..].to_vec(),
                            )) {
                                let value = rec_solve(
                                    ThreeValue::MustNot,
                                    seq.clone()[1..].to_string(),
                                    &ins[1..],
                                    memo_hash,
                                );
                                memo_hash.insert(
                                    (
                                        ThreeValue::MustNot,
                                        seq.clone()[1..].to_string(),
                                        ins[1..].to_vec(),
                                    ),
                                    value,
                                );
                            }
                            *memo_hash
                                .get(&(
                                    ThreeValue::MustNot,
                                    seq.clone()[1..].to_string(),
                                    ins[1..].to_vec(),
                                ))
                                .unwrap()
                        }
                    }
                    ThreeValue::MustNot => {
                        if !memo_hash.contains_key(&(
                            ThreeValue::Can,
                            seq.clone()[1..].to_string(),
                            ins.to_vec(),
                        )) {
                            let value = rec_solve(
                                ThreeValue::Can,
                                seq.clone()[1..].to_string(),
                                ins,
                                memo_hash,
                            );
                            memo_hash.insert(
                                (ThreeValue::Can, seq.clone()[1..].to_string(), ins.to_vec()),
                                value,
                            );
                        }
                        *memo_hash
                            .get(&(ThreeValue::Can, seq.clone()[1..].to_string(), ins.to_vec()))
                            .unwrap()
                    }
                    ThreeValue::Can => {
                        if ins.is_empty() {
                            if !memo_hash.contains_key(&(
                                ThreeValue::MustNot,
                                seq.clone()[1..].to_string(),
                                ins.to_vec(),
                            )) {
                                let value = rec_solve(
                                    ThreeValue::MustNot,
                                    seq.clone()[1..].to_string(),
                                    ins,
                                    memo_hash,
                                );
                                memo_hash.insert(
                                    (
                                        ThreeValue::MustNot,
                                        seq.clone()[1..].to_string(),
                                        ins.to_vec(),
                                    ),
                                    value,
                                );
                            }
                            return *memo_hash
                                .get(&(
                                    ThreeValue::MustNot,
                                    seq.clone()[1..].to_string(),
                                    ins.to_vec(),
                                ))
                                .unwrap();
                        }
                        let a = if ins[0] > 1 {
                            if !memo_hash.contains_key(&(
                                ThreeValue::Must,
                                replace_char_at_index(&seq, 0, '#'),
                                ins.to_vec(),
                            )) {
                                let value = rec_solve(
                                    ThreeValue::Must,
                                    replace_char_at_index(&seq, 0, '#'),
                                    ins,
                                    memo_hash,
                                );
                                memo_hash.insert(
                                    (
                                        ThreeValue::Must,
                                        replace_char_at_index(&seq, 0, '#'),
                                        ins.to_vec(),
                                    ),
                                    value,
                                );
                            }
                            *memo_hash
                                .get(&(
                                    ThreeValue::Must,
                                    replace_char_at_index(&seq, 0, '#'),
                                    ins.to_vec(),
                                ))
                                .unwrap()
                        } else {
                            if !memo_hash.contains_key(&(
                                ThreeValue::MustNot,
                                seq.clone()[1..].to_string(),
                                ins[1..].to_vec(),
                            )) {
                                let value = rec_solve(
                                    ThreeValue::MustNot,
                                    seq.clone()[1..].to_string(),
                                    &ins[1..],
                                    memo_hash,
                                );
                                memo_hash.insert(
                                    (
                                        ThreeValue::MustNot,
                                        seq.clone()[1..].to_string(),
                                        ins[1..].to_vec(),
                                    ),
                                    value,
                                );
                            }
                            *memo_hash
                                .get(&(
                                    ThreeValue::MustNot,
                                    seq.clone()[1..].to_string(),
                                    ins[1..].to_vec(),
                                ))
                                .unwrap()
                        };
                        let b = {
                            if !memo_hash.contains_key(&(
                                ThreeValue::Can,
                                seq.clone()[1..].to_string(),
                                ins.to_vec(),
                            )) {
                                let value = rec_solve(
                                    ThreeValue::Can,
                                    seq.clone()[1..].to_string(),
                                    ins,
                                    memo_hash,
                                );
                                memo_hash.insert(
                                    (ThreeValue::Can, seq.clone()[1..].to_string(), ins.to_vec()),
                                    value,
                                );
                            }
                            *memo_hash
                                .get(&(ThreeValue::Can, seq.clone()[1..].to_string(), ins.to_vec()))
                                .unwrap()
                        };
                        //rec_solve(ThreeValue::Can, &seq[1..].to_string(), ins, memo_hash);
                        a + b
                    }
                },
                _ => unreachable!(),
            },
        }
    }

    rec_solve(ThreeValue::Can, seq.to_string(), &ins, &mut memo_hash)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (seq, ins) = line.split_once(' ').unwrap();
        let ins = ins
            .split(',')
            .map(|c| c.parse::<usize>().ok().unwrap())
            .collect::<Vec<usize>>();
        sum += solve(seq, ins)
    }
    Some(sum as u32)
}

fn unfold(sequence: Vec<char>, groups: Vec<usize>, n: usize) -> (Vec<char>, Vec<usize>) {
    let seq_len = sequence.len();
    let grp_len = groups.len();

    let new_sequence = sequence
        .into_iter()
        .chain(once('?'))
        .cycle()
        .take(seq_len * n + n - 1)
        .collect();
    let new_groups = groups.into_iter().cycle().take(grp_len * n).collect();

    (new_sequence, new_groups)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let (seq, ins) = line.split_once(' ').unwrap();
        let (seq, ins) = unfold(
            seq.chars().collect_vec(),
            ins.split(',')
                .map(|c| c.parse::<usize>().ok().unwrap())
                .collect::<Vec<usize>>(),
            5,
        );
        sum += solve(seq.iter().collect::<String>().as_str(), ins)
    }
    Some(sum as usize)
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
