use std::collections::{HashMap, VecDeque};

trait Module {
    fn tick(&mut self, signal: Pulse, name: String);
    fn store(&mut self, _: String) {}
    fn get_state(&self) -> Pulse;
    fn cont(&self, _: Pulse) -> bool {
        true
    }
}

#[derive(PartialEq, Clone, Copy, Default, Debug, Hash, Eq)]
enum Pulse {
    #[default]
    Low,
    High,
}

use Pulse::*;

impl Pulse {
    fn invert(&self) -> Pulse {
        match self {
            Low => High,
            High => Low,
        }
    }
}

impl std::ops::Not for Pulse {
    type Output = Pulse;
    fn not(self) -> Pulse {
        self.invert()
    }
}

#[derive(Default, Clone)]
struct FlipFlop {
    state: Pulse,
}

impl Module for FlipFlop {
    fn tick(&mut self, signal: Pulse, _: String) {
        if signal == Low {
            self.state = !self.state;
        }
    }
    fn cont(&self, signal: Pulse) -> bool {
        if signal == High {
            return false;
        }
        true
    }
    fn get_state(&self) -> Pulse {
        self.state
    }
}

#[derive(Default, Clone)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
    state: Pulse,
}

impl Module for Conjunction {
    fn tick(&mut self, signal: Pulse, name: String) {
        self.inputs.insert(name, signal);
        self.state = match self.inputs.iter().all(|(_, &p)| p == High) {
            true => Low,
            false => High,
        };
    }
    fn store(&mut self, name: String) {
        self.inputs.insert(name, Low);
    }
    fn get_state(&self) -> Pulse {
        self.state
    }
}

#[derive(Default, Clone)]
struct Broadcaster {
    state: Pulse,
}

impl Module for Broadcaster {
    fn tick(&mut self, signal: Pulse, _: String) {
        self.state = signal;
    }
    fn get_state(&self) -> Pulse {
        self.state
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut node_type = HashMap::new();
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (src, dest) = line.split_once(" -> ").unwrap();
        let (module, name): (Box<dyn Module>, &str) = match src.split_at(1) {
            ("%", src) => (Box::new(FlipFlop::default()), src),
            ("&", src) => (Box::new(Conjunction::default()), src),
            ("b", _src) => (Box::new(Broadcaster::default()), "broadcaster"),
            _ => unreachable!(),
        };
        let targets: Vec<String> = dest.split(", ").map(|s| s.to_string()).collect();
        node_type.insert(name.to_string(), module);
        graph.insert(name.to_string(), targets);
        for (src, targets) in graph.iter() {
            for target in targets {
                node_type.get_mut(target).map(|n| n.store(src.clone()));
            }
        }
    }
    let mut hp = 0;
    let mut lp = 0;
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        lp += 1;
        queue.push_back(("broadcaster".to_string(), Low));
        while let Some((v, signal)) = queue.pop_front() {
            for u in graph.get(&v).unwrap() {
                match signal {
                    Low => lp += 1,
                    High => hp += 1,
                }
                if u == "rx" || u == "output" {
                    continue;
                }
                let target = node_type.get_mut(u).unwrap();
                if !target.cont(signal) {
                    continue;
                }
                target.tick(signal, v.clone());
                queue.push_back((u.to_string(), target.get_state()));
            }
        }
    }
    Some(lp * hp)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut node_type = HashMap::new();
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (src, dest) = line.split_once(" -> ").unwrap();
        let (module, name): (Box<dyn Module>, &str) = match src.split_at(1) {
            ("%", src) => (Box::new(FlipFlop::default()), src),
            ("&", src) => (Box::new(Conjunction::default()), src),
            ("b", _src) => (Box::new(Broadcaster::default()), "broadcaster"),
            _ => unreachable!(),
        };
        let targets: Vec<String> = dest.split(", ").map(|s| s.to_string()).collect();
        node_type.insert(name.to_string(), module);
        graph.insert(name.to_string(), targets);
        for (src, targets) in graph.iter() {
            for target in targets {
                node_type.get_mut(target).map(|n| n.store(src.clone()));
            }
        }
    }
    let mut res = Vec::new();
    for i in 0..4000 {
        let mut queue = VecDeque::new();
        queue.push_back(("broadcaster".to_string(), Low));
        while let Some((v, signal)) = queue.pop_front() {
            for u in graph.get(&v).unwrap() {
                if u == "ql" && signal == High {
                    res.push(i + 1);
                    continue;
                }
                if u == "rx" {
                    continue;
                }
                let target = node_type.get_mut(u).unwrap();
                if !target.cont(signal) {
                    continue;
                }
                target.tick(signal, v.clone());
                queue.push_back((u.to_string(), target.get_state()));
            }
        }
    }
    Some(res.iter().fold(1, |acc, &num| lcm(acc as u64, num as u64)))
}

aoc::solution!(20);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 20)),
            Some(32000000)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 20)), None);
    }
}
