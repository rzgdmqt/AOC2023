use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Tuple(String, String, usize, String),
    String(String),
}

fn check_rule(rules: &Vec<Rule>, part: &HashMap<String, Vec<usize>>) -> Option<String> {
    for rule in rules {
        if let Rule::Tuple(var, cond, num, dest) = rule {
            if (cond == "<" && part[var][0] < *num) || (cond == ">" && part[var][0] > *num) {
                return Some(dest.clone());
            }
        } else if let Rule::String(rule_str) = rule {
            return Some(rule_str.clone());
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();

    let re1 = r"(\w+)\{(.+)\}";
    let re2 = r"(\w)(<|>)(\d+):(\w+)";

    let mut result = 0;

    let (lines1, lines2) = input.split_once("\n\n").unwrap();

    for l in lines1.lines() {
        let caps = regex::Regex::new(re1).unwrap().captures(l).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let captures = caps.get(2).unwrap().as_str();
        let mut chain = Vec::new();

        for part in captures.split(',') {
            let caps2 = regex::Regex::new(re2).unwrap().captures(part);
            if let Some(caps2) = caps2 {
                let var = caps2.get(1).unwrap().as_str().to_string();
                let cond = caps2.get(2).unwrap().as_str();
                let num = caps2.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let dest = caps2.get(4).unwrap().as_str().to_string();
                chain.push(Rule::Tuple(var, cond.to_string(), num, dest));
            } else {
                chain.push(Rule::String(part.to_string()));
            }
        }

        workflows.insert(name, chain);
    }

    for l in lines2.lines() {
        let mut part: HashMap<String, Vec<usize>> = HashMap::new();
        for val in l[1..l.len() - 1].split(',') {
            let var_val: Vec<&str> = val.split('=').collect();
            let var = var_val[0].to_string();
            let num = var_val[1].parse::<usize>().unwrap();
            part.insert(var, vec![num]);
        }

        let mut rule = String::from("in");
        while rule != "A" && rule != "R" {
            rule = check_rule(&workflows[&rule], &part).unwrap_or("".to_string());
        }

        if rule == "A" {
            result += part.values().flatten().sum::<usize>();
        }
    }

    Some(result as usize)
}

fn check_rule_range(
    workflows: &HashMap<String, Vec<Rule>>,
    rules: &Vec<Rule>,
    part: &mut HashMap<String, Vec<usize>>,
) -> usize {
    let mut output = 0;

    if let Some(firstrule) = rules.first() {
        let (_, rest) = rules.split_at(1);

        if let Rule::Tuple(var, cond, num, dest) = firstrule {
            let r = part.get(var).unwrap().clone();
            let r_true: Vec<usize> = match cond.as_str() {
                "<" => (r[0]..(num - 1).min(r[r.len() - 1]) + 1).collect(),
                ">" => ((num + 1).max(r[0])..r[r.len() - 1] + 1).collect(),
                _ => unreachable!(),
            };
            let r_false: Vec<usize> = match cond.as_str() {
                "<" => (*num.max(&r[0])..r[r.len() - 1] + 1).collect(),
                ">" => (r[0]..*num.min(&r[r.len() - 1]) + 1).collect(),
                _ => unreachable!(),
            };

            let mut part_true = part.clone();
            part_true.insert(var.clone(), r_true.clone());

            let mut part_false = part.clone();
            part_false.insert(var.clone(), r_false.clone());

            if !r_true.is_empty() {
                if dest == "A" {
                    output += part_true
                        .values()
                        .map(|v| v.len() as usize)
                        .product::<usize>();
                } else if dest != "R" {
                    output += check_rule_range(workflows, &workflows[dest], &mut part_true);
                }
            }

            if !r_false.is_empty() {
                output += check_rule_range(workflows, &rest.to_vec(), &mut part_false);
            }
        } else if let Rule::String(rule_str) = firstrule {
            if rule_str == "A" {
                output += part.values().map(|v| v.len() as usize).product::<usize>();
            } else if rule_str != "R" {
                output += check_rule_range(workflows, &workflows[rule_str], part);
            }
        }
    }

    output
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();

    let re1 = r"(\w+)\{(.+)\}";
    let re2 = r"(\w)(<|>)(\d+):(\w+)";

    let (lines1, _lines2) = input.split_once("\n\n").unwrap();

    for l in lines1.lines() {
        let caps = regex::Regex::new(re1).unwrap().captures(l).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let captures = caps.get(2).unwrap().as_str();
        let mut chain = Vec::new();

        for part in captures.split(',') {
            let caps2 = regex::Regex::new(re2).unwrap().captures(part);
            if let Some(caps2) = caps2 {
                let var = caps2.get(1).unwrap().as_str().to_string();
                let cond = caps2.get(2).unwrap().as_str();
                let num = caps2.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let dest = caps2.get(4).unwrap().as_str().to_string();
                chain.push(Rule::Tuple(var, cond.to_string(), num, dest));
            } else {
                chain.push(Rule::String(part.to_string()));
            }
        }

        workflows.insert(name, chain);
    }

    let mut part: HashMap<String, Vec<usize>> = HashMap::new();
    part.insert("x".to_string(), (1..=4000).collect());
    part.insert("m".to_string(), (1..=4000).collect());
    part.insert("a".to_string(), (1..=4000).collect());
    part.insert("s".to_string(), (1..=4000).collect());

    Some(check_rule_range(&workflows, &workflows["in"], &mut part))
}

aoc::solution!(19);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 19)),
            Some(19114)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 19)),
            Some(167409079868000)
        );
    }
}
