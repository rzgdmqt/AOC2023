use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut itr = line.chars().filter(char::is_ascii_digit);
                let fst = itr.next().unwrap();
                let lst = itr.last().unwrap_or(fst);
                format!("{fst}{lst}").parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9)").unwrap();
    let re_rev =
        Regex::new(r"(9|8|7|6|5|4|3|2|1|0|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();
    Some(
        input
            .lines()
            .map(|line| {
                let mut itr_fst = re.captures_iter(line).map(|c| c.extract());
                let rev_str: String = line.chars().rev().collect();
                let mut itr_lst = re_rev.captures_iter(&rev_str).map(|c| c.extract());
                let (_, [fst]) = itr_fst.next().unwrap();
                let (_, [lst]) = itr_lst.next().unwrap();
                let lst_chars: String = lst.chars().rev().collect();
                mapper(fst) * 10 + mapper(&lst_chars)
            })
            .sum(),
    )
}

fn mapper(num: &str) -> u32 {
    match num {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => unreachable!(),
    }
}

aoc::solution!(1);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 1)),
            Some(253)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 1)),
            Some(281)
        );
    }
}
