pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut sum = 0;
    for line in lines {
        let mut all_ok = true;
        let space = line.find(' ').unwrap();
        let colon = line.find(':').unwrap();
        let id = &line[space + 1..colon];
        let games = line[colon + 2..].split("; ");
        'game_loop: for game in games {
            let colors = game.split(", ");
            for color in colors {
                let num_col: Vec<&str> = color.split(' ').collect();
                if num_col[1] == "red" && num_col[0].parse::<u32>().ok().unwrap() > red
                    || num_col[1] == "blue" && num_col[0].parse::<u32>().ok().unwrap() > blue
                    || num_col[1] == "green" && num_col[0].parse::<u32>().ok().unwrap() > green
                {
                    all_ok = false;
                    break 'game_loop;
                }
            }
        }
        if all_ok {
            sum += id.parse::<u32>().ok().unwrap()
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let mut min_red = 1;
        let mut min_blue = 1;
        let mut min_green = 1;
        let colon = line.find(':').unwrap();
        let games = line[colon + 2..].split("; ");
        for game in games {
            let colors = game.split(", ");
            for color in colors {
                let num_col: Vec<&str> = color.split(' ').collect();
                if num_col[1] == "red" {
                    if min_red > 1 {
                        min_red = min_red.max(num_col[0].parse::<u32>().ok().unwrap())
                    } else {
                        min_red = num_col[0].parse::<u32>().ok().unwrap()
                    }
                }
                if num_col[1] == "blue" {
                    if min_blue > 1 {
                        min_blue = min_blue.max(num_col[0].parse::<u32>().ok().unwrap())
                    } else {
                        min_blue = num_col[0].parse::<u32>().ok().unwrap()
                    }
                }
                if num_col[1] == "green" {
                    if min_green > 1 {
                        min_green = min_green.max(num_col[0].parse::<u32>().ok().unwrap())
                    } else {
                        min_green = num_col[0].parse::<u32>().ok().unwrap()
                    }
                }
            }
        }
        sum += min_blue * min_green * min_red;
    }
    Some(sum)
}

aoc::solution!(2);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 2)), Some(8));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 2)),
            Some(2286)
        );
    }
}
