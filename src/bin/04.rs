use std::collections::HashSet;

fn card_worth(winning: Vec<&str>, my: Vec<&str>) -> u32 {
    let my_set: HashSet<&str> = my.into_iter().collect();
    let winning_set: HashSet<&str> = winning.into_iter().collect();
    let mut overlap: HashSet<&str> = my_set.intersection(&winning_set).cloned().collect();
    overlap.remove("");
    if overlap.is_empty() {
        0
    } else {
        2_u32.pow(overlap.len() as u32 - 1)
    }
}

fn match_number(winning: Vec<&str>, my: Vec<&str>) -> usize {
    let my_set: HashSet<&str> = my.into_iter().collect();
    let winning_set: HashSet<&str> = winning.into_iter().collect();
    let mut overlap: HashSet<&str> = my_set.intersection(&winning_set).cloned().collect();
    overlap.remove("");
    overlap.len()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let colon = line.find(": ").unwrap();
        let pipe = line.find(" | ").unwrap();
        let winning_numbers: Vec<&str> = (line[colon + 2..pipe]).split(' ').collect();
        let my_numbers: Vec<&str> = (line[pipe + 3..]).split(' ').collect();
        sum += card_worth(winning_numbers, my_numbers);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut amounts: Vec<u32> = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let colon = line.find(": ").unwrap();
        let pipe = line.find(" | ").unwrap();
        let winning_numbers: Vec<&str> = (line[colon + 2..pipe]).split(' ').collect();
        let my_numbers: Vec<&str> = (line[pipe + 3..]).split(' ').collect();
        let match_number = match_number(winning_numbers, my_numbers);
        for j in (i + 1)..=(i + match_number) {
            amounts[j] += amounts[i];
        }
    }
    Some(amounts.iter().sum())
}

aoc::solution!(4);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 4)), Some(13));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 4)), Some(30));
    }
}
