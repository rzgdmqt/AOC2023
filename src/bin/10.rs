use itertools::{enumerate, Itertools};

fn posible_step(grid: &Vec<Vec<char>>, current: (usize, usize), next: (usize, usize)) -> bool {
    match grid[current.0][current.1] {
        '|' => {
            (['L', 'J', 'S', '|'].contains(&grid[next.0][next.1]) && next.0 == current.0 + 1
                || ['F', '7', 'S', '|'].contains(&grid[next.0][next.1]) && next.0 == current.0 - 1)
                && (next.1 == current.1)
        }
        '-' => {
            (['7', 'J', 'S', '-'].contains(&grid[next.0][next.1]) && next.1 == current.1 + 1
                || ['L', 'F', 'S', '-'].contains(&grid[next.0][next.1]) && next.1 == current.1 - 1)
                && (next.0 == current.0)
        }
        'L' => {
            ['7', '|', 'S', 'F'].contains(&grid[next.0][next.1])
                && next.0 == current.0 - 1
                && (next.1 == current.1)
                || ['-', 'J', '7', 'S'].contains(&grid[next.0][next.1])
                    && next.1 == current.1 + 1
                    && (next.0 == current.0)
        }
        'J' => {
            ['7', '|', 'S', 'F'].contains(&grid[next.0][next.1])
                && next.0 == current.0 - 1
                && (next.1 == current.1)
                || ['-', 'L', 'F', 'S'].contains(&grid[next.0][next.1])
                    && next.1 == current.1 - 1
                    && (next.0 == current.0)
        }
        '7' => {
            ['-', 'L', 'S', 'F'].contains(&grid[next.0][next.1])
                && next.1 == current.1 - 1
                && (next.0 == current.0)
                || ['|', 'L', 'J', 'S'].contains(&grid[next.0][next.1])
                    && next.0 == current.0 + 1
                    && (next.1 == current.1)
        }
        'F' => {
            ['-', 'J', 'S', '7'].contains(&grid[next.0][next.1])
                && next.1 == current.1 + 1
                && (next.0 == current.0)
                || ['|', 'L', 'J', 'S'].contains(&grid[next.0][next.1])
                    && next.0 == current.0 + 1
                    && (next.1 == current.1)
        }
        'S' => {
            ['-', 'J', '7'].contains(&grid[next.0][next.1])
                && next.1 == current.1 + 1
                && (next.0 == current.0)
                || ['-', 'L', 'F'].contains(&grid[next.0][next.1])
                    && next.1 == current.1 - 1
                    && (next.0 == current.0)
                || ['|', 'J', 'L'].contains(&grid[next.0][next.1])
                    && next.0 == current.0 + 1
                    && (next.1 == current.1)
                || ['|', '7', 'F'].contains(&grid[next.0][next.1])
                    && next.0 == current.0 - 1
                    && (next.1 == current.1)
        }
        '.' => false,
        _ => unreachable!(),
    }
}

fn cycle_from_start(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    dir: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut visited = vec![];
    let start_init = start;
    let mut possible_dir = vec![dir];
    let mut next = start;
    while !possible_dir.is_empty() {
        next = possible_dir.pop().unwrap();
        visited.push(next);
        let prev = start;
        let start = next;
        let steps: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (i, j) in steps {
            if start.0 as isize + i < 0
                || start.0 as isize + i >= grid.len() as isize
                || start.1 as isize + j < 0
                || start.1 as isize + j >= grid[0].len() as isize
            {
                continue;
            }
            next = (
                (start.0 as isize + i) as usize,
                (start.1 as isize + j) as usize,
            );
            if posible_step(grid, start, next) {
                if !visited.contains(&next) && next != prev {
                    possible_dir.push(next);
                }
            }
        }
    }
    visited.push(start);
    visited
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start = (0, 0);
    let mut grid = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut vec = Vec::new();
        for (j, char) in line.chars().enumerate() {
            vec.push(char);
            if char == 'S' {
                start = (i, j);
            }
        }
        grid.push(vec);
    }
    let steps = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (i, j) in steps {
        if start.0 as isize + i < 0
            || start.0 as isize + i >= grid.len() as isize
            || start.1 as isize + j < 0
            || start.1 as isize + j >= grid[0].len() as isize
        {
            continue;
        }
        let next = (
            (start.0 as isize + i) as usize,
            (start.1 as isize + j) as usize,
        );
        if posible_step(&grid, start, next) {
            let cycle = cycle_from_start(&grid, start, next);
            return Some(cycle.len() as u32 / 2);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut inner = 0;
    let mut start = (0, 0);
    let mut grid = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut vec = Vec::new();
        for (j, char) in line.chars().enumerate() {
            vec.push(char);
            if char == 'S' {
                start = (i, j);
            }
        }
        grid.push(vec);
    }
    let steps = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (i, j) in steps {
        if start.0 as isize + i < 0
            || start.0 as isize + i >= grid.len() as isize
            || start.1 as isize + j < 0
            || start.1 as isize + j >= grid[0].len() as isize
        {
            continue;
        }
        let next = (
            (start.0 as isize + i) as usize,
            (start.1 as isize + j) as usize,
        );
        if posible_step(&grid, start, next) {
            let cycle = cycle_from_start(&grid, start, next);
            for (i, line) in input.lines().enumerate() {
                for (j, _) in line.chars().enumerate() {
                    if cycle.contains(&(i, j)) {
                        continue;
                    }
                    let mut sum = 0;
                    let mut same_row = cycle.iter().filter(|&&c| c.0 == i && c.1 < j).collect_vec();
                    same_row.sort();
                    let mut sequence = Vec::new();
                    for (row, column) in same_row.iter() {
                        sequence.push(grid[*row][*column]);
                    }
                    let mut tmp_sequence = vec!['p'];
                    for char in sequence {
                        if char == '|' {
                            sum += 1;
                            tmp_sequence.push('|');
                        } else if char == '-' {
                            continue;
                        } else if char == 'L' {
                            sum += 1;
                            tmp_sequence.push('L');
                        } else if char == 'F' {
                            sum += 1;
                            tmp_sequence.push('F');
                        } else if (char == 'J' || char == 'S')
                            && *tmp_sequence.last().unwrap() == 'L'
                        {
                            tmp_sequence.pop();
                            sum -= 1;
                        } else if (char == 'J' || char == 'S')
                            && *tmp_sequence.last().unwrap() == 'F'
                        {
                            tmp_sequence.pop();
                        } else if char == '7' && *tmp_sequence.last().unwrap() == 'L' {
                            tmp_sequence.pop();
                        } else if char == '7' && *tmp_sequence.last().unwrap() == 'F' {
                            tmp_sequence.pop();
                            sum -= 1;
                        }
                    }
                    if sum % 2 == 1 {
                        inner += 1
                    }
                }
            }
            return Some(inner);
        }
    }
    None
}

aoc::solution!(10);

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_part_one() {
    //     assert_eq!(part_one(&aoc::template::read_file("examples", 10)), Some(8));
    // }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 10)), Some(8));
    }
}
