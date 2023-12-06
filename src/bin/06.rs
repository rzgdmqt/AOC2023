pub fn part_one(input: &str) -> Option<u32> {
    let mut ways_total = 1;
    let mut time = Vec::new();
    let mut dist = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            for num in line.split_whitespace() {
                if num.parse::<usize>().is_ok() {
                    time.push(num.parse::<usize>().ok().unwrap())
                }
            }
        }
        if i == 1 {
            for num in line.split_whitespace() {
                if num.parse::<usize>().is_ok() {
                    dist.push(num.parse::<usize>().ok().unwrap())
                }
            }
        }
    }
    for (time, dist) in time.iter().zip(dist.iter()) {
        let mut ways_single = 0;
        for i in 0..=*time {
            let my_distance = i * (time - i);
            if my_distance > *dist {
                ways_single += 1
            }
        }
        ways_total *= ways_single
    }
    Some(ways_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (line1, line2) = input.split_once('\n').unwrap();
    let time = line1.replace(' ', "").parse::<usize>().ok().unwrap();
    let dist = line2.replace(' ', "").parse::<usize>().ok().unwrap();
    let mut ways_single = 0;
    for i in 0..=time {
        let my_distance = i * (time - i);
        if my_distance > dist {
            ways_single += 1
        }
    }
    Some(ways_single)
}

aoc::solution!(6);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 6)),
            Some(288)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 6)),
            Some(71503)
        );
    }
}
