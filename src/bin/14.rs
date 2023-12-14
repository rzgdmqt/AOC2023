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

fn roll(orig_grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid = orig_grid.clone();
    for j in 0..grid[0].len() {
        let mut move_pos = 0;
        for i in 0..grid.len() {
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

fn rotate_clockwise(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
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
fn weight(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for j in 0..grid[0].len() {
        for i in 0..grid.len() {
            match grid[i][j] {
                'O' => sum += grid.len() - i,
                _ => continue,
            }
        }
    }
    sum
}

fn solve(input: &str) -> Option<u32> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    let mut result = Vec::new();
    for _ in 0..200 {
        let total = weight(&grid);
        result.push(total);
        for _ in 0..4 {
            grid = roll(&grid);
            grid = rotate_clockwise(&grid);
        }
    }
    let mut period = 0;
    for i in (0..result.len() - 2).rev() {
        if result[i] == result[result.len() - 1] {
            period = (result.len() - 1) - i;
            break;
        }
    }
    let remainred = 1000000000 % period;
    for i in (0..result.len() - 1).rev() {
        if i % period == remainred {
            return Some(result[i] as u32);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let a = solve(input).unwrap();
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
