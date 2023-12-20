use std::collections::BinaryHeap;

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    fn index(&self) -> usize {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }
}

#[derive(PartialEq, Eq)]
struct Node {
    pos: usize,
    dir: Option<Direction>,
    distance: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

fn solve<const MIN: usize, const MAX: usize>(grid: Vec<usize>, cols: usize, rows: usize) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = vec![(false, usize::MAX); grid.len() * 4 * MAX];
    heap.push(Node {
        pos: 0,
        dir: None,
        distance: 0,
        cost: 0,
    });
    while let Some(Node {
        pos,
        dir,
        distance,
        cost,
    }) = heap.pop()
    {
        match dir {
            Some(d) => visited[pos * 4 * MAX + d.index() * MAX + distance].0 = true,
            None => {
                for d in 0..4 {
                    visited[pos * 4 * MAX + d * MAX + distance].0 = true
                }
            }
        };
        heap.extend([North, East, South, West].iter().filter_map(|&d| {
            let (same_dir, opposite_dir) = match dir {
                Some(dd) => (dd == d, dd.opposite() == d),
                None => (true, false),
            };
            let outside = match d {
                North => pos / rows == 0,
                East => pos % cols == cols - 1,
                South => pos / rows == rows - 1,
                West => pos % cols == 0,
            };
            if (distance < MIN && !same_dir)
                || (distance > MAX - 1 && same_dir)
                || opposite_dir
                || outside
            {
                return None;
            }
            let new_pos = match d {
                North => pos - cols,
                East => pos + 1,
                South => pos + cols,
                West => pos - 1,
            };
            let new_dist = 1 + if same_dir { distance } else { 0 };
            let new_dest = new_pos * 4 * MAX + d.index() * MAX + new_dist;
            let new_cost = cost + grid[new_pos];
            let (is_visited, prevcost) = visited[new_dest];
            if is_visited || prevcost <= new_cost {
                return None;
            }
            visited[new_dest].1 = new_cost;
            Some(Node {
                pos: new_pos,
                dir: Some(d),
                distance: new_dist,
                cost: new_cost,
            })
        }));
    }
    visited[(grid.len() - 1) * 4 * MAX..]
        .iter()
        .map(|(_, cost)| *cost)
        .min()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cols = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .flatten()
        .collect_vec();
    let rows = grid.len() / cols;
    Some(solve::<0, 3>(grid, cols, rows) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cols = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .flatten()
        .collect_vec();
    let rows = grid.len() / cols;
    Some(solve::<4, 10>(grid, cols, rows) as u32)
}

aoc::solution!(17);

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 17)),
            Some(102)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 17)),
            Some(94)
        );
    }
}
