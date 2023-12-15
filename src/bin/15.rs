fn hash(string: &str) -> u32 {
    let mut tmp = 0;
    for c in string.chars() {
        tmp = ((tmp + c as u32) * 17) % 256;
    }
    tmp
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for string in input.split(',') {
        sum += hash(string)
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![Vec::new(); 256];
    for string in input.split(',') {
        let (address, length): (&str, Option<u32>) = match string.contains('=') {
            true => {
                let splited: Vec<&str> = string.split('=').collect();
                (splited[0], Some(splited[1].parse::<u32>().ok().unwrap()))
            }
            false => (&string[0..string.len() - 1], None),
        };
        let mut updated = false;
        for (i, (addr, _)) in boxes[hash(address) as usize].clone().iter().enumerate() {
            if addr == &address {
                match length {
                    Some(v) => {
                        boxes[hash(address) as usize][i] = (address, v);
                        updated = true;
                    }
                    None => {
                        boxes[hash(address) as usize].remove(i);
                        break;
                    }
                }
            }
        }
        if !updated & length.is_some() {
            boxes[hash(address) as usize].push((address, length.unwrap()))
        }
    }
    for (i, box_content) in boxes.iter().enumerate() {
        for (j, (_, len)) in box_content.iter().enumerate() {
            sum += (i + 1) * (j + 1) * *len as usize
        }
    }
    Some(sum as u32)
}

aoc::solution!(15);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 15)),
            Some(1320)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 15)),
            Some(145)
        );
    }
}
