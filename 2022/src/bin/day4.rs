use std::cmp::{max, min};

fn main() {
    let input = include_str!("../../data/day4.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let v = l
                .split(',')
                .flat_map(|range| range.split('-').map(|v| v.parse::<u32>().unwrap()))
                .collect::<Vec<_>>();
            if (v[0] >= v[2] && v[1] <= v[3]) || (v[2] >= v[0] && v[3] <= v[1]) {
                return Some(());
            }

            None
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let v = l
                .split(',')
                .flat_map(|range| range.split('-').map(|v| v.parse::<i32>().unwrap()))
                .collect::<Vec<_>>();
            match max(v[0], v[2]) <= min(v[1], v[3]) {
                true => Some(()),
                false => None,
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    #[test]
    fn day3_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
