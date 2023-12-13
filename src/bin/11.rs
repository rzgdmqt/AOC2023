fn step_range(vec: &Vec<char>, start: usize, end: usize, step: usize) -> Vec<char> {
    let mut v = Vec::new();
    for i in (start..end).step_by(step) {
        v.push(vec[i]);
    }
    v
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut orig_galaxy = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    let mut empty_rows = Vec::new();
    for (i, line) in input.lines().enumerate() {
        rows += 1;
        cols = 0;
        if line.chars().all(|c| c == '.') {
            empty_rows.push(i)
        }
        for c in line.chars() {
            cols += 1;
            orig_galaxy.push(c);
        }
    }
    let mut empty_cols = Vec::new();
    for i in 0..cols {
        if !step_range(&orig_galaxy, i, rows * cols, cols).contains(&'#') {
            empty_cols.push(i);
        }
    }

    let mut points = Vec::new();
    for (i, c) in orig_galaxy.iter().enumerate() {
        if c == &'#' {
            points.push(((i / cols), i % cols))
        }
    }
    let mut sum = 0;
    for (i, (x1, y1)) in points.iter().enumerate() {
        for (x2, y2) in (points[i + 1..]).into_iter() {
            for j in *(x1.min(x2))..*(x1.max(x2)) {
                if empty_rows.contains(&j) {
                    sum += 2;
                } else {
                    sum += 1;
                }
            }
            for j in *(y1.min(y2))..*(y1.max(y2)) {
                if empty_cols.contains(&j) {
                    sum += 2;
                } else {
                    sum += 1;
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut orig_galaxy = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    let mut empty_rows = Vec::new();
    for (i, line) in input.lines().enumerate() {
        rows += 1;
        cols = 0;
        if line.chars().all(|c| c == '.') {
            empty_rows.push(i)
        }
        for c in line.chars() {
            cols += 1;
            orig_galaxy.push(c);
        }
    }
    let mut empty_cols = Vec::new();
    for i in 0..cols {
        if !step_range(&orig_galaxy, i, rows * cols, cols).contains(&'#') {
            empty_cols.push(i);
        }
    }

    let mut points = Vec::new();
    for (i, c) in orig_galaxy.iter().enumerate() {
        if c == &'#' {
            points.push(((i / cols), i % cols))
        }
    }
    let mut sum = 0;
    for (i, (x1, y1)) in points.iter().enumerate() {
        for (x2, y2) in (points[i + 1..]).into_iter() {
            for j in *(x1.min(x2))..*(x1.max(x2)) {
                if empty_rows.contains(&j) {
                    sum += 1000000;
                } else {
                    sum += 1;
                }
            }
            for j in *(y1.min(y2))..*(y1.max(y2)) {
                if empty_cols.contains(&j) {
                    sum += 1000000;
                } else {
                    sum += 1;
                }
            }
        }
    }
    Some(sum)
}

aoc::solution!(11);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 11)),
            Some(374)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 11)),
            Some(82000210)
        );
    }
}
