use std::collections::HashSet;

use itertools::Itertools;

enum Move {
    North,
    South,
    East,
    West,
}

use Move::*;

impl Move {
    fn change_pos(&self, curr: usize, cols: usize) -> usize {
        match self {
            North => curr - cols,
            South => curr + cols,
            East => curr + 1,
            West => curr - 1,
        }
    }
    fn grid_move(&self, (currx, curry): (isize, isize)) -> (isize, isize) {
        match self {
            North => (currx - 1, curry),
            South => (currx + 1, curry),
            East => (currx, curry + 1),
            West => (currx, curry - 1),
        }
    }
}

fn get_ref(x: isize, y: isize) -> (isize, isize) {
    let cyclic_x = ((x % 131) + 131) % 131;
    let cyclic_y = ((y % 131) + 131) % 131;
    (cyclic_x, cyclic_y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let cols = input.lines().next().unwrap().len();
    let mut grid: Vec<char> = input
        .lines()
        .map(|line| line.chars())
        .flatten()
        .collect_vec();
    let rows = grid.len() / cols;
    let mut starts = HashSet::new();
    starts.insert(grid.iter().position(|&c| c == 'S').unwrap());
    grid = grid
        .iter()
        .map(|&c| if c == 'S' { '.' } else { c })
        .collect_vec();
    let mut new_starts;
    for _ in 0..64 {
        new_starts = HashSet::new();
        for pos in starts.iter() {
            for dir in vec![North, South, East, West] {
                let outside = match dir {
                    North => pos / rows == 0,
                    East => pos % cols == cols - 1,
                    South => pos / rows == rows - 1,
                    West => pos % cols == 0,
                };
                if outside {
                    continue;
                }
                let new_pos = dir.change_pos(*pos, cols);
                if grid[new_pos] == '#' {
                    continue;
                }
                new_starts.insert(new_pos);
            }
        }
        starts = new_starts;
    }
    Some(starts.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total = 26501365;
    let mut starts = HashSet::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut v = Vec::new();
        for (j, c) in line.chars().enumerate() {
            v.push(c);
            if c == 'S' {
                starts.insert((i as isize, j as isize));
            }
        }
        grid.push(v);
    }
    let rows = grid.len();
    let mut coef = Vec::new();
    let mut new_starts;
    for i in 1.. {
        new_starts = HashSet::new();
        for pos in starts.iter() {
            for dir in vec![North, South, East, West] {
                let (new_x, new_y) = dir.grid_move(*pos);
                let (ref_x, ref_y) = get_ref(new_x, new_y);
                if grid[ref_x as usize][ref_y as usize] == '#' {
                    continue;
                }
                new_starts.insert((new_x, new_y));
            }
        }
        starts = new_starts;
        if i % rows == total % rows {
            println!("iteration: {i} \tresult: {}", starts.len());
            coef.push(starts.len());
            if coef.len() == 3 {
                break;
            }
        }
    }

    let c = coef[0];
    let a = (coef[2] - 2 * coef[1] + coef[0]) / 2;
    let b = (coef[1] - coef[0]) - a;
    let x = (total - (total % rows)) / rows;
    Some((a * x * x + b * x + c) as u64)
}

aoc::solution!(21);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 21)),
            Some(42)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 21)), None);
    }
}
