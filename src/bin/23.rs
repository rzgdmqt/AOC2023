use std::collections::HashSet;
use Dir::*;

#[derive(Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}

fn dfs(
    graph: &[char],
    cols: usize,
    u: usize,
    mut path: Vec<usize>,
    mut visited: HashSet<usize>,
    mut maximum: usize,
) -> usize {
    if u == graph.len() - 2 {
        return maximum.max(path.len());
    }

    for dir in &[N, S, E, W] {
        let current = graph[u];
        let v = match dir {
            N => {
                if u < cols || ['v', '<', '>'].contains(&current) {
                    continue;
                }
                u - cols
            }
            S => {
                if u + cols >= graph.len() || ['^', '<', '>'].contains(&current) {
                    continue;
                }
                u + cols
            }
            E => {
                if u + 1 >= graph.len() || ['^', '<', 'v'].contains(&current) {
                    continue;
                }
                u + 1
            }
            W => {
                if u == 0 || ['^', 'v', '>'].contains(&current) {
                    continue;
                }
                u - 1
            }
        };
        if graph[v] == '#' || visited.contains(&v) {
            continue;
        }
        path.push(v);
        visited.insert(v);
        maximum = dfs(graph, cols, v, path.clone(), visited.clone(), maximum).max(maximum);
        visited.remove(&v);
        path.pop();
    }
    maximum
}

pub fn part_one(input: &str) -> Option<u32> {
    let cols = input.lines().next().unwrap().len();
    let graph: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    Some(dfs(&graph, cols, 1, vec![1], HashSet::new(), 0) as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .replace('<', ".")
        .replace('>', ".")
        .replace('v', ".")
        .replace('^', ".");
    let cols = input.lines().next().unwrap().len();
    let graph: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    Some(dfs(&graph, cols, 1, vec![1], HashSet::new(), 0) as u32 - 1)
}

aoc::solution!(23);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 23)),
            Some(94)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 23)),
            Some(154)
        );
    }
}
