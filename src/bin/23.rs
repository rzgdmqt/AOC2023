use std::collections::HashSet;
use Dir::*;

#[derive(Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}

fn neighbours(graph: &[char], cols: usize, source: usize) -> Vec<usize> {
    let mut neighbours = Vec::new();
    for dir in &[N, S, E, W] {
        let current = graph[source];
        let v = match dir {
            N => {
                if source < cols || ['v', '<', '>'].contains(&current) {
                    continue;
                }
                source - cols
            }
            S => {
                if source + cols >= graph.len() || ['^', '<', '>'].contains(&current) {
                    continue;
                }
                source + cols
            }
            E => {
                if source + 1 >= graph.len() || ['^', '<', 'v'].contains(&current) {
                    continue;
                }
                source + 1
            }
            W => {
                if source == 0 || ['^', 'v', '>'].contains(&current) {
                    continue;
                }
                source - 1
            }
        };
        if graph[v] != '#' {
            neighbours.push(v);
        }
    }
    neighbours
}

fn collapse(
    graph: &[char],
    cols: usize,
    mut source: usize,
    mut neighbour: usize,
    mut distance: usize,
) -> (usize, usize) {
    let mut n = neighbours(graph, cols, neighbour);
    while n.len() == 2 {
        n.retain(|&s| s != source);
        source = neighbour;
        neighbour = n[0];
        distance += 1;
        n = neighbours(graph, cols, neighbour);
    }
    (neighbour, distance)
}

fn dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    head: usize,
    distance: usize,
    mut maximum: usize,
    mut visited: HashSet<usize>,
) -> usize {
    if head == graph.len() - 2 {
        return distance;
    } else if visited.contains(&head) {
        return maximum;
    }
    visited.insert(head);
    for (n, d) in graph[head].iter() {
        maximum = maximum.max(dfs(graph, *n, distance + *d, maximum, visited.clone()))
    }
    visited.remove(&head);
    maximum
}

pub fn part_one(input: &str) -> Option<u32> {
    let cols = input.lines().next().unwrap().len();
    let graph: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let mut collapsed_graph = vec![Vec::new(); graph.len()];
    for i in 0..graph.len() {
        if graph[i] != '#' {
            let nn = neighbours(&graph, cols, i);
            for n in nn {
                let aaa = collapse(&graph, cols, i, n, 1);
                collapsed_graph[i].push(aaa);
            }
        }
    }
    Some(dfs(&collapsed_graph, 1, 0, 0, HashSet::new()) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .replace('<', ".")
        .replace('>', ".")
        .replace('v', ".")
        .replace('^', ".");
    let cols = input.lines().next().unwrap().len();
    let graph: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let mut collapsed_graph = vec![Vec::new(); graph.len()];
    for i in 0..graph.len() {
        if graph[i] != '#' {
            let nn = neighbours(&graph, cols, i);
            for n in nn {
                let aaa = collapse(&graph, cols, i, n, 1);
                collapsed_graph[i].push(aaa);
            }
        }
    }
    Some(dfs(&collapsed_graph, 1, 0, 0, HashSet::new()) as u32)
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
