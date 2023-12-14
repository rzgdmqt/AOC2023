use std::{collections::HashSet, ops::Index};

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut rows = 0;
    let mut cols = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        cols = 0;
        let mut row = Vec::new();
        rows += 1;
        for c in line.chars() {
            cols += 1;
            row.push(c);
        }
        grid.push(row);
    }
    for j in 0..cols {
        let mut current_weight = cols;
        for i in 0..rows {
            match grid[i][j] {
                'O' => {
                    sum += current_weight;
                    current_weight -= 1;
                }
                '#' => current_weight = rows - i - 1,
                '.' => continue,
                _ => unreachable!(),
            }
        }
    }
    Some(sum as u32)
}

fn roll(orig_grid: Vec<Vec<char>>, rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid = orig_grid.clone();
    for j in 0..cols {
        let mut move_pos = 0;
        for i in 0..rows {
            match orig_grid[i][j] {
                'O' => {
                    grid[move_pos][j] = 'O';
                    if move_pos != i {
                        grid[i][j] = '.';
                    }
                    move_pos += 1;
                }
                '#' => move_pos = i + 1,
                '.' => continue,
                _ => unreachable!(),
            }
        }
    }
    grid
}

fn rotate_clockwise(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut result = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            result[j][rows - i - 1] = grid[i][j];
        }
    }
    result
}
fn weight(grid: Vec<Vec<char>>, rows: usize, cols: usize) -> usize {
    let mut sum = 0;
    for j in 1..cols {
        for i in 1..rows {
            match grid[i][j] {
                'O' => sum += rows - i,
                _ => continue,
            }
        }
    }
    sum
}
fn solve(input: &str, n: u32) -> Option<u32> {
    let mut rows = 0;
    let mut cols = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        cols = 0;
        let mut row = Vec::new();
        rows += 1;
        for c in line.chars() {
            cols += 1;
            row.push(c);
        }
        grid.push(row);
    }
    for i in 0..n {
        for _ in 0..4 {
            let mut new_grid = roll(grid.clone(), rows, cols);
            new_grid = rotate_clockwise(new_grid);
            let tmp = rows;
            rows = cols;
            cols = tmp;
            grid = new_grid;
        }
    }
    Some(weight(grid, rows, cols) as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    let a = solve(input, 101).unwrap();
    Some(a)
}

aoc::solution!(14);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 14)),
            Some(136)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 14)),
            Some(64)
        );
    }
}

