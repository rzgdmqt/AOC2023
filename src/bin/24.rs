fn intersection(
    (a, b, c): (f64, f64, f64),
    (va, vb, vc): (f64, f64, f64),
    (x, y, z): (f64, f64, f64),
    (vx, vy, vz): (f64, f64, f64),
) -> (f64, f64, f64) {
    let mu = (va * b + x * vb - a * vb - y * va) / (va * vy - vx * vb);
    let lambda = (x - a + mu * vx) / va;
    if mu < 0. || lambda < 0. {
        return (0., 0., 0.);
    }
    let fst = x + mu * vx;
    let snd = y + mu * vy;
    let trd = z + mu * vz;
    (fst, snd, trd)
}

pub fn part_one(input: &str) -> Option<u32> {
    const MIN: f64 = 200000000000000.;
    // const MIN: f64 = 7.;
    const MAX: f64 = 400000000000000.;
    // const MAX: f64 = 27.;
    let data: Vec<((f64, f64, f64), (f64, f64, f64))> = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let [x, y, z] = pos
                .splitn(3, ", ")
                .map(|c| c.trim().parse().ok().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let [vx, vy, vz] = vel
                .splitn(3, ", ")
                .map(|c| c.trim().parse().ok().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            ((x, y, z), (vx, vy, vz))
        })
        .collect();
    let mut count = 0;
    for (pos1, vel1) in data.iter() {
        for (pos2, vel2) in data.iter() {
            if pos1 == pos2 {
                continue;
            }
            let (x, y, _) = intersection(*pos1, *vel1, *pos2, *vel2);
            if MIN <= x && x <= MAX && MIN <= y && y <= MAX {
                count += 1;
            }
        }
    }
    Some(count / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

aoc::solution!(24);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 24)), Some(2));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 24)), None);
    }
}
