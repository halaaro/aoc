use std::collections::HashMap;

fn main() {
    let input = include_str!("../../data/day14.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct V2(i32, i32);

impl V2 {
    fn dir(&self) -> V2 {
        // assumes only single direction
        V2(self.0 / self.0.abs().max(1), self.1 / self.1.abs().max(1))
    }
}

impl std::ops::Sub for V2 {
    type Output = V2;

    fn sub(self, rhs: Self) -> Self::Output {
        V2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add for V2 {
    type Output = V2;

    fn add(self, rhs: Self) -> Self::Output {
        V2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn part1(input: &str) -> usize {
    let mut map = build_map(input);

    let mut old_count = -1;
    let mut count = 0;

    while count > old_count {
        old_count = count;
        add_sand(&mut map, V2(500, 0));
        count = map.values().filter(|&&v| v == 'o').count() as i32;
    }

    count as usize
}

fn part2(input: &str) -> usize {
    let mut map = build_map(input);

    let bottom = map.keys().map(|k| k.1).max().unwrap() + 2;
    let mut count = 0;
    while !map.contains_key(&V2(500, 0)) {
        add_sand2(&mut map, V2(500, 0), bottom);
        count += 1;
    }

    // print map
    // let imin = map.keys().map(|k| k.0).min().unwrap();
    // let imax = map.keys().map(|k| k.0).max().unwrap();
    // let jmax = map.keys().map(|k| k.1).max().unwrap();
    // for j in 0..=jmax + 1 {
    //     println!(
    //         "{}",
    //         (imin-1..=imax+1)
    //             .map(|i| map.get(&V2(i, j)).unwrap_or(&'.'))
    //             .collect::<String>()
    //     );
    // }

    count as usize
}

fn build_map(input: &str) -> HashMap<V2, char> {
    let obstacles = input
        .lines()
        .map(|l| l.split(" -> ").map(|c| c.split_once(',').unwrap()));
    let mut map = HashMap::new();
    // fill in obstacles
    for obs in obstacles {
        let mut last_pt = None;
        for pt in obs {
            let pt = V2(pt.0.parse().unwrap(), pt.1.parse().unwrap());
            if let Some(mut last_pt) = last_pt {
                let dir = (pt - last_pt).dir();
                map.entry(last_pt).or_insert('#');
                while pt != last_pt {
                    last_pt = last_pt + dir;
                    map.entry(last_pt).or_insert('#');
                }
            }
            last_pt = Some(pt);
        }
    }
    map
}

fn add_sand(map: &mut HashMap<V2, char>, mut pos: V2) {
    let bottom = map.keys().map(|k| k.1).max().unwrap();

    while pos.1 < bottom {
        if map.get(&V2(pos.0, pos.1 + 1)).is_none() {
            pos.1 += 1;
            continue;
        }
        if map.get(&V2(pos.0 - 1, pos.1 + 1)).is_none() {
            pos.0 -= 1;
            pos.1 += 1;
            continue;
        }
        if map.get(&V2(pos.0 + 1, pos.1 + 1)).is_none() {
            pos.0 += 1;
            pos.1 += 1;
            continue;
        }

        map.entry(pos).or_insert('o');
        break;
    }
}

fn add_sand2(map: &mut HashMap<V2, char>, mut pos: V2, bottom: i32) {
    loop {
        if pos.1 == bottom - 1 {
            break;
        }
        if map.get(&V2(pos.0, pos.1 + 1)).is_none() {
            pos.1 += 1;
            continue;
        }
        if map.get(&V2(pos.0 - 1, pos.1 + 1)).is_none() {
            pos.0 -= 1;
            pos.1 += 1;
            continue;
        }
        if map.get(&V2(pos.0 + 1, pos.1 + 1)).is_none() {
            pos.0 += 1;
            pos.1 += 1;
            continue;
        }
        break;
    }
    map.entry(pos).or_insert('o');
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 93);
    }
}
