use itertools::Itertools;

fn area(border: &Vec<(isize, isize)>, border_length: isize) -> isize {
    let mut area: isize = 0;
    for n in 0..border.len() {
        let (i1, j1) = border[n];
        let (i2, j2) = border[(n + 1) % border.len()];
        area += (i1 + i2) * (j1 - j2);
    }
    area / 2 + border_length / 2 + 1
}

fn border(instructions: &Vec<(&str, isize)>) -> (Vec<(isize, isize)>, isize) {
    let mut border = Vec::new();
    let mut len = 0;
    let (mut i, mut j) = (0, 0);
    border.push((i, j));

    for (d, l) in instructions.iter() {
        let (di, dj) = match d {
            &"U" => (-1, 0),
            &"D" => (1, 0),
            &"R" => (0, 1),
            &"L" => (0, -1),
            _ => unreachable!(),
        };
        (i, j) = (i + l * di, j + l * dj);
        border.push((i, j));
        len += l;
    }

    border.pop();

    (border, len)
}

pub fn part_one(input: &str) -> Option<isize> {
    let instructions: Vec<(&str, isize)> = input
        .lines()
        .map(|line| {
            let splited = line.split(' ').collect_vec();
            (splited[0], splited[1].parse::<isize>().ok().unwrap())
        })
        .collect();

    let (border, border_length) = border(&instructions);
    Some(area(&border, border_length))
}

pub fn part_two(input: &str) -> Option<isize> {
    let instructions: Vec<(&str, isize)> = input
        .lines()
        .map(|line| {
            let splited = line.split(' ').collect_vec();
            let dir = match &splited[2][7..8] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => unreachable!(),
            };
            (
                dir,
                isize::from_str_radix(&splited[2][2..7], 16).ok().unwrap(),
            )
        })
        .collect();

    let (border, border_length) = border(&instructions);
    Some(area(&border, border_length))
}

aoc::solution!(18);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 18)),
            Some(62)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 18)),
            Some(952408144115)
        );
    }
}
