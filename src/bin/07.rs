use itertools::Itertools;
use std::{collections::HashMap, usize};

fn determine_rank(cards: Vec<usize>) -> u32 {
    let mut counter: [u32; 15] = [0; 15];
    for card in cards {
        counter[card] += 1
    }
    if counter.contains(&5) {
        7
    } else if counter.contains(&4) {
        6
    } else if counter.contains(&3) && counter.contains(&2) {
        5
    } else if counter.contains(&3) {
        4
    } else if counter.iter().filter(|&&x| x == 2).count() == 2 {
        3
    } else if counter.iter().filter(|&&x| x == 2).count() == 1 {
        2
    } else if counter.iter().sum::<u32>() == 5 {
        1
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut values: HashMap<char, u32> = HashMap::new();
    values.insert('T', 10);
    values.insert('J', 11);
    values.insert('Q', 12);
    values.insert('K', 13);
    values.insert('A', 14);
    let mut cards: Vec<(Vec<usize>, u32)> = Vec::new();
    for line in input.lines() {
        let splited: Vec<&str> = line.split(' ').collect();
        let hand = splited[0]
            .chars()
            .map(|c| {
                if c.is_ascii_digit() {
                    c.to_string().parse::<usize>().ok().unwrap()
                } else {
                    *values.get(&c).unwrap() as usize
                }
            })
            .collect();
        let bid = splited[1].parse::<u32>().ok().unwrap();
        cards.push((hand, bid));
    }
    cards.sort();
    let mut final_cards = Vec::new();
    for (hand, bid) in cards {
        let num = determine_rank(hand.clone());
        final_cards.push((num, hand, bid))
    }
    final_cards.sort();
    let mut total: u32 = 0;
    for (i, (_, _, bid)) in final_cards.iter().enumerate() {
        total += *bid * (i as u32 + 1);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut values: HashMap<char, u32> = HashMap::new();
    let all_options = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14];
    values.insert('T', 10);
    values.insert('J', 0);
    values.insert('Q', 12);
    values.insert('K', 13);
    values.insert('A', 14);
    let mut cards: Vec<(Vec<usize>, u32)> = Vec::new();
    for line in input.lines() {
        let splited: Vec<&str> = line.split(' ').collect();
        let hand = splited[0]
            .chars()
            .map(|c| {
                if c.is_ascii_digit() {
                    c.to_string().parse::<usize>().ok().unwrap()
                } else {
                    *values.get(&c).unwrap() as usize
                }
            })
            .collect();
        let bid = splited[1].parse::<u32>().ok().unwrap();
        cards.push((hand, bid));
    }
    cards.sort();
    let mut final_cards = Vec::new();
    for (hand, bid) in cards {
        let mut without_j = hand.clone();
        without_j.retain(|x| *x != 0);
        let all_possible_j: Vec<Vec<usize>> = all_options
            .clone()
            .into_iter()
            .combinations_with_replacement(5 - without_j.len())
            .collect();
        let mut max_rank = 0;
        for posibility in all_possible_j {
            let mut tmp_cards = without_j.clone();
            tmp_cards.extend(posibility);
            max_rank = max_rank.max(determine_rank(tmp_cards))
        }
        final_cards.push((max_rank, hand, bid))
    }
    final_cards.sort();
    let mut total: u32 = 0;
    for (i, (_, _, bid)) in final_cards.iter().enumerate() {
        total += *bid * (i as u32 + 1);
    }
    Some(total)
}

aoc::solution!(7);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 7)),
            Some(6440)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 7)),
            Some(5905)
        );
    }
}
