use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../../data/day12.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}
fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().map(move |(j, c)| {
                (
                    (i as i32, j as i32),
                    match c {
                        'S' => -1,
                        'a'..='z' => c as i8 - 97,
                        'E' => 26,
                        _ => todo!(),
                    },
                )
            })
        })
        .collect::<HashMap<_, _>>();

    let start = map.iter().find(|(_, e)| **e == -1).unwrap().0;

    let mut dist = HashMap::new();
    dist.insert(*start, 0);
    let mut queue = VecDeque::new();
    queue.push_front((*start, map[start], 0));

    // let mut cnt = 0;
    let mut min_steps = usize::MAX;
    let mut visited = HashSet::new();

    while let Some(((i, j), v, steps)) = queue.pop_back() {
        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));
        if v == 26 {
            min_steps = min_steps.min(steps);
            continue;
        }
        for dir in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
            if let Some(&v2) = map.get(&dir) {
                if v2 <= v + 1 {
                    let d = dist.entry(dir).or_insert(usize::MAX);
                    *d = (*d).min(steps + 1);
                    queue.push_front((dir, v2, *d));
                }
            }
        }
    }

    min_steps
}

fn part2(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().map(move |(j, c)| {
                (
                    (i as i32, j as i32),
                    match c {
                        'S' => -1,
                        'a'..='z' => c as i8 - 97,
                        'E' => 26,
                        _ => todo!(),
                    },
                )
            })
        })
        .collect::<HashMap<_, _>>();

    let starts = map
        .iter()
        .filter_map(|(ij, e)| if *e == 0 { Some(ij) } else { None });

    let mut min_steps = usize::MAX;
    for start in starts {
        let mut dist = HashMap::new();
        dist.insert(*start, 0);
        let mut queue = VecDeque::new();
        queue.push_front((*start, map[start], 0));

        // let mut cnt = 0;
        let mut visited = HashSet::new();

        while let Some(((i, j), v, steps)) = queue.pop_back() {
            if visited.contains(&(i, j)) {
                continue;
            }
            visited.insert((i, j));
            if v == 26 {
                min_steps = min_steps.min(steps);
                continue;
            }
            for dir in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if let Some(&v2) = map.get(&dir) {
                    if v2 <= v + 1 {
                        let d = dist.entry(dir).or_insert(usize::MAX);
                        *d = (*d).min(steps + 1);
                        queue.push_front((dir, v2, *d));
                    }
                }
            }
        }
    }

    min_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 31);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 29);
    }
}
