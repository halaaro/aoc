use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("../../data/day15.input");
    println!("part1: {}", part1(input, 2000000));
    println!("part2: {}", part2(input, 4000000));
}

trait INums<T> {
    fn inums(&self) -> Vec<T>;
}

// generic "inumber" parser, just because I wanted one
impl<T> INums<T> for &str
where
    T: FromStr + Copy,
{
    fn inums(&self) -> Vec<T> {
        let mut start = None;
        let mut end = None;
        let mut nums = vec![];
        for (i, c) in self.char_indices() {
            match (start, c) {
                // start of number
                (None, '-' | '0'..='9') => start = Some(i),
                // middle of number
                (Some(_), '0'..='9') => (),
                // done
                (Some(_), _) => end = Some(i),
                _ => (),
            }
            if i == &self.len() - 1 {
                end = Some(i + 1)
            }
            if let (Some(istart), Some(iend)) = (start, end) {
                if istart == iend {
                    continue;
                }
                if let Ok(i) = &self[istart..iend].parse() {
                    nums.push(*i)
                }
                (start, end) = (None, None);
            }
        }
        nums
    }
}

fn part1(input: &str, row: usize) -> usize {
    let row = row as i32;
    let positions = input.lines().map(|l| {
        let nums = l.inums();
        ((nums[0], nums[1]), (nums[2], nums[3]))
    });

    let mut coverage = positions
        .clone()
        .into_iter()
        .map(|(s, b)| (s, b, manhattan(s, b)))
        .filter_map(|(s, _, m)| {
            //  calc dist to y={row} from sensor, return range within m
            let covered = m - (s.1 - row).abs();
            if covered > 0 {
                Some((s.0 - covered, s.0 + covered))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    coverage.sort();

    // collapse ranges
    let coverage = coverage
        .into_iter()
        .fold(vec![], |mut acc: Vec<(i32, i32)>, c| {
            if let Some(last) = acc.last_mut() {
                if last.1 >= c.0 {
                    // merge
                    *last = (last.0, last.1.max(c.1));
                    return acc;
                }
            }
            acc.push(c);
            acc
        });

    let total_coverage: i32 = coverage.into_iter().map(|c| c.1 - c.0 + 1).sum();
    let beacons = positions
        .map(|(_s, b)| b)
        .filter(|b| b.1 == row)
        .collect::<HashSet<_>>()
        .len();
    total_coverage as usize - beacons
}

fn part2(input: &str, max: i32) -> usize {
    let positions: Vec<_> = input
        .lines()
        .map(|l| {
            let nums: Vec<i32> = l.inums();
            ((nums[0], nums[1]), (nums[2], nums[3]))
        })
        .map(|(s, b)| (s, b, manhattan(s, b)))
        .collect();

    for row in 0..=max {
        let mut coverage: Vec<_> = positions
            .iter()
            // find coverage
            .filter_map(|(s, _b, m)| match m - (s.1 - row).abs() {
                c if c > 0 => Some((s.0 - c, s.0 + c)),
                _ => None,
            })
            .collect();
        coverage.sort();
        let first_cov_ends =
            coverage
                .into_iter()
                .fold(0i32, |acc, c| if acc >= c.0 { acc.max(c.1) } else { acc });
        if first_cov_ends < max {
            return (first_cov_ends as usize + 1) * 4000000 + row as usize;
        }
    }
    0
}

fn manhattan(s: (i32, i32), b: (i32, i32)) -> i32 {
    (s.0 - b.0).abs() + (s.1 - b.1).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn day15_part1() {
        assert_eq!(part1(TEST_INPUT, 10), 26);
    }
    #[test]
    fn day15_part2() {
        assert_eq!(part2(TEST_INPUT, 20), 56000011);
    }
}
