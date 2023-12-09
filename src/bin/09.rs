use itertools::Itertools;

fn to_differences(input: Vec<i64>) -> Vec<i64> {
    let mut output = Vec::new();
    if input.len() == 1 {
        output.push(0_i64);
        return output;
    }
    for i in 1..input.len() {
        output.push(input[i] - input[i - 1]);
    }
    output
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut history: Vec<i64> = Vec::new();
    for line in input.lines() {
        let mut last_num = Vec::new();
        let mut numbers: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|c| c.parse().ok().unwrap())
            .collect_vec();
        while !numbers.iter().all(|&num| num == 0) {
            last_num.push(*numbers.last().unwrap());
            numbers = to_differences(numbers);
        }
        history.push(last_num.iter().sum())
    }
    Some(history.iter().sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut history: Vec<i64> = Vec::new();
    for line in input.lines() {
        let mut last_num = Vec::new();
        let mut numbers: Vec<i64> = line
            .split_ascii_whitespace()
            .rev()
            .map(|c| c.parse().ok().unwrap())
            .collect_vec();
        while !numbers.iter().all(|&num| num == 0) {
            last_num.push(*numbers.last().unwrap());
            numbers = to_differences(numbers);
        }
        history.push(last_num.iter().sum())
    }
    Some(history.iter().sum())
}

aoc::solution!(9);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 9)),
            Some(114)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 9)), Some(2));
    }
}
