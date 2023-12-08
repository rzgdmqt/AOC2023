use std::collections::HashMap;

use num::integer::gcd;

pub fn part_one(input: &str) -> Option<u32> {
    let mut steps = 0;
    let (seq, ins) = input.split_once("\n\n").unzip();
    let mut instructions: HashMap<&str, [&str; 2]> = HashMap::new();
    let seq: Vec<usize> = seq
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .collect();
    for line in ins.unwrap().lines() {
        let index_space = line.find(' ').unwrap();
        let index_bra = line.find('(').unwrap();
        let index_comma = line.find(',').unwrap();
        let index_cket = line.find(')').unwrap();
        let k = &line[0..index_space];
        let v = [
            &line[index_bra + 1..index_comma],
            &line[index_comma + 2..index_cket],
        ];
        instructions.insert(k, v);
    }
    let mut start = "AAA";
    let mut cycled_ins = seq.iter().cycle();
    while start != "ZZZ" {
        start = instructions.get(start).unwrap()[*cycled_ins.next().unwrap()];
        steps += 1
    }
    Some(steps)
}

fn lcm(a: u128, b: u128) -> u128 {
    a / gcd(a, b) * b
}

fn lcm_of_list(list: &[u128]) -> u128 {
    list.iter().cloned().fold(1, lcm)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (seq, ins) = input.split_once("\n\n").unzip();
    let mut instructions: HashMap<&str, [&str; 2]> = HashMap::new();
    let mut starts = Vec::new();
    let seq: Vec<usize> = seq
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .collect();
    for line in ins.unwrap().lines() {
        let index_space = line.find(' ').unwrap();
        let index_bra = line.find('(').unwrap();
        let index_comma = line.find(',').unwrap();
        let index_cket = line.find(')').unwrap();
        let k = &line[0..index_space];
        let v = [
            &line[index_bra + 1..index_comma],
            &line[index_comma + 2..index_cket],
        ];
        if k.ends_with('A') {
            starts.push(k);
        }
        instructions.insert(k, v);
    }
    let mut lenghts = Vec::new();
    for start in starts.iter() {
        let mut steps = 0;
        let mut cycled_ins = seq.iter().cycle();
        let mut tmp_start = *start;
        while !tmp_start.ends_with('Z') {
            let index = *cycled_ins.next().unwrap();
            tmp_start = instructions[tmp_start][index];
            steps += 1
        }
        lenghts.push(steps)
    }
    Some(lcm_of_list(&lenghts))
}
aoc::solution!(8);

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_part_one() {
    //     assert_eq!(part_one(&aoc::template::read_file("examples", 8)), Some(2));
    // }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 8)), Some(6));
    }
}
