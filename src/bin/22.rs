use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    fn min_x(&self, other: &Coordinate) -> usize {
        self.x.min(other.x)
    }
    fn min_y(&self, other: &Coordinate) -> usize {
        self.y.min(other.y)
    }
    fn min_z(&self, other: &Coordinate) -> usize {
        self.z.min(other.z)
    }
    fn max_x(&self, other: &Coordinate) -> usize {
        self.x.max(other.x)
    }
    fn max_y(&self, other: &Coordinate) -> usize {
        self.y.max(other.y)
    }
    fn max_z(&self, other: &Coordinate) -> usize {
        self.z.max(other.z)
    }
    fn move_down(&self) -> Coordinate {
        Coordinate {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

fn intersect(r1: (usize, Coordinate, Coordinate), r2: (usize, Coordinate, Coordinate)) -> bool {
    let brx1 = r1.1.max_x(&r1.2);
    let bry1 = r1.1.max_y(&r1.2);
    let tlx1 = r1.1.min_x(&r1.2);
    let tly1 = r1.1.min_y(&r1.2);
    let brx2 = r2.1.max_x(&r2.2);
    let bry2 = r2.1.max_y(&r2.2);
    let tlx2 = r2.1.min_x(&r2.2);
    let tly2 = r2.1.min_y(&r2.2);
    let x_overlap = brx1 >= tlx2 && tlx1 <= brx2;
    let y_overlap = bry1 >= tly2 && tly1 <= bry2;

    x_overlap && y_overlap
}

fn intersect_any(
    coord: (usize, Coordinate, Coordinate),
    coords: &Vec<(usize, Coordinate, Coordinate)>,
    ignore: usize,
) -> bool {
    let coords = coords.iter().filter(|(i, _, _)| *i != ignore);
    for r2 in coords {
        if intersect(coord, *r2) {
            return true;
        }
    }
    false
}

fn fall_down(
    layers: &Vec<Vec<(usize, Coordinate, Coordinate)>>,
    ignore: usize,
) -> (
    Vec<Vec<(usize, Coordinate, Coordinate)>>,
    bool,
    HashSet<usize>,
) {
    let mut change = false;
    let mut fallen = HashSet::new();
    let mut new_layers = vec![Vec::new(); layers.len()];
    new_layers[0] = layers[0].clone();
    for i in 0..&layers.len() - 1 {
        let current_layer = new_layers[i].clone();
        let next_layer = &layers[i + 1].clone();
        for r1 in next_layer {
            if r1.0 == ignore {
                continue;
            }
            if !intersect_any(*r1, &current_layer, ignore) {
                change = true;
                let new_r1 = (r1.0, r1.1.move_down(), r1.2.move_down());
                new_layers[i].push(new_r1);
                fallen.insert(r1.0);
            } else {
                new_layers[i + 1].push(*r1)
            }
        }
    }

    (new_layers, change, fallen)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks_num = 0;
    let mut coordinate_pairs: Vec<(usize, Coordinate, Coordinate)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line.split_once('~').unwrap();
            let xyz1 = start.split(',').collect_vec();
            let xyz2 = end.split(',').collect_vec();
            bricks_num = i + 1;
            (
                i + 1,
                Coordinate {
                    x: xyz1[0].parse::<usize>().ok().unwrap(),
                    y: xyz1[1].parse::<usize>().ok().unwrap(),
                    z: xyz1[2].parse::<usize>().ok().unwrap() - 1,
                },
                Coordinate {
                    x: xyz2[0].parse::<usize>().ok().unwrap(),
                    y: xyz2[1].parse::<usize>().ok().unwrap(),
                    z: xyz2[2].parse::<usize>().ok().unwrap() - 1,
                },
            )
        })
        .collect();
    let (_, maxz1, maxz2) = coordinate_pairs
        .iter()
        .max_by_key(|(_, c1, c2)| c1.z.max(c2.z))
        .unwrap();
    let maxz = maxz1.max_z(maxz2);
    coordinate_pairs.sort_by_key(|(_, c1, c2)| c1.min_z(c2));
    let mut layers = vec![Vec::new(); maxz + 1];
    for (i, c1, c2) in coordinate_pairs {
        for layer in c1.min_z(&c2)..=c1.max_z(&c2) {
            layers[layer].push((i, c1, c2))
        }
    }
    let (mut new_layers, mut change, _) = fall_down(&layers, usize::MAX);
    while change {
        (new_layers, change, _) = fall_down(&new_layers, usize::MAX);
    }
    let mut count = 0;
    for i in 1..=bricks_num {
        let (_, change, fallen) = fall_down(&new_layers, i);
        if !change {
            count += 1
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bricks_num = 0;
    let mut coordinate_pairs: Vec<(usize, Coordinate, Coordinate)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line.split_once('~').unwrap();
            let xyz1 = start.split(',').collect_vec();
            let xyz2 = end.split(',').collect_vec();
            bricks_num = i + 1;
            (
                i + 1,
                Coordinate {
                    x: xyz1[0].parse::<usize>().ok().unwrap(),
                    y: xyz1[1].parse::<usize>().ok().unwrap(),
                    z: xyz1[2].parse::<usize>().ok().unwrap() - 1,
                },
                Coordinate {
                    x: xyz2[0].parse::<usize>().ok().unwrap(),
                    y: xyz2[1].parse::<usize>().ok().unwrap(),
                    z: xyz2[2].parse::<usize>().ok().unwrap() - 1,
                },
            )
        })
        .collect();
    let (_, maxz1, maxz2) = coordinate_pairs
        .iter()
        .max_by_key(|(_, c1, c2)| c1.z.max(c2.z))
        .unwrap();
    let maxz = maxz1.max_z(maxz2);
    coordinate_pairs.sort_by_key(|(_, c1, c2)| c1.min_z(c2));
    let mut layers = vec![Vec::new(); maxz + 1];
    for (i, c1, c2) in coordinate_pairs {
        for layer in c1.min_z(&c2)..=c1.max_z(&c2) {
            layers[layer].push((i, c1, c2))
        }
    }
    let (mut new_layers, mut change, _) = fall_down(&layers, usize::MAX);
    while change {
        (new_layers, change, _) = fall_down(&new_layers, usize::MAX);
    }
    let mut count = 0;
    for i in 1..=bricks_num {
        let mut all_fallen = HashSet::new();
        let (mut new_layers, mut change, mut fallen) = fall_down(&new_layers, i);
        all_fallen.extend(fallen.clone());
        while change {
            (new_layers, change, fallen) = fall_down(&new_layers, i);
            all_fallen.extend(fallen.clone());
        }
        count += all_fallen.len();
    }
    Some(count as u32)
}

aoc::solution!(22);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 22)), Some(5));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 22)), Some(7));
    }
}
