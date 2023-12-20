use std::collections::HashSet;

fn solve(grid: &Vec<Vec<char>>, start: (isize, isize), dir: (isize, isize)) -> Option<u32> {
    let mut seen: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    let move_dir = (start, dir);
    let mut starts: Vec<((isize, isize), (isize, isize))> = vec![move_dir];
    while !starts.is_empty() {
        let ((mut i, mut j), (mut di, mut dj)) = starts.pop().unwrap();
        if i < 0 || i >= grid.len() as isize || j < 0 || j >= grid[0].len() as isize {
            continue;
        }
        while !seen.contains(&(i, j, di, dj)) {
            seen.insert((i, j, di, dj));
            let (new_di, new_dj) = match grid[i as usize][j as usize] {
                '/' => {
                    if (di, dj) == (1, 0) || (di, dj) == (-1, 0) {
                        (dj, -di)
                    } else {
                        (-dj, di)
                    }
                }
                '\\' => {
                    if (di, dj) == (1, 0) || (di, dj) == (-1, 0) {
                        (dj, di)
                    } else {
                        (dj, di)
                    }
                }
                '-' => {
                    if (di, dj) == (1, 0) || (di, dj) == (-1, 0) {
                        starts.push(((i, (j + di)), (dj, di)));
                        (dj, -di)
                    } else {
                        (di, dj)
                    }
                }
                '|' => {
                    if (di, dj) == (0, 1) || (di, dj) == (0, -1) {
                        starts.push((((i + dj), j), (dj, di)));
                        (-dj, di)
                    } else {
                        (di, dj)
                    }
                }
                _ => (di, dj),
            };
            let (new_i, new_j) = (i + new_di, j + new_dj);
            if new_i < 0
                || new_i >= grid.len() as isize
                || new_j < 0
                || new_j >= grid[0].len() as isize
            {
                break;
            }
            ((i, j), (di, dj)) = ((new_i, new_j), (new_di, new_dj));
        }
    }
    let seen: HashSet<_> = seen.into_iter().map(|(i, j, _, _)| (i, j)).collect();
    Some(seen.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();
    solve(&grid, (0, 0), (0, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();
    let mut max = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if i > 0 && i < grid.len() - 1 && (j != 0 || j != grid[0].len()) {
                continue;
            }
            let start = (i as isize, j as isize);
            let dirs: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            for &dir in dirs.iter() {
                max = max.max(solve(&grid, start, dir).unwrap())
            }
        }
    }
    Some(max)
}

aoc::solution!(16);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 16)),
            Some(46)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 16)),
            Some(51)
        );
    }
}
