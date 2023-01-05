#![allow(unused)]
use std::{
    cmp::max,
    io::{stdout, Write},
    str::FromStr,
};

fn main() {
    let input = include_str!("../../data/day19.input");
    println!("\npart1: {}", part1(input));
    println!("\npart2: {}", part2(input));
}

#[derive(Default, Debug)]
struct Blueprint {
    line: String,
    num: usize,
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: (usize, usize),
    geode_robot: (usize, usize),
}

impl Blueprint {
    fn calc_max_geodes(&self, maxmin: usize) -> usize {
        self.max_geodes_recursive([0, 0, 0, 0], [1, 0, 0, 0], 1, maxmin)
    }

    fn max_geodes_recursive(
        &self,
        res: [usize; 4],
        robots: [usize; 4],
        min: usize,
        maxmin: usize,
    ) -> usize {
        if min > maxmin {
            return res[3];
        }

        let mut max_geodes = res[3];

        if res[0] >= self.geode_robot.0 && res[2] >= self.geode_robot.1 {
            let mut _res = res;
            _res[0] -= self.geode_robot.0;
            _res[2] -= self.geode_robot.1;

            let _res = update_res(_res, robots);

            let mut _robots = robots;
            _robots[3] += 1;
            max_geodes = max(
                max_geodes,
                self.max_geodes_recursive(_res, _robots, min + 1, maxmin),
            );
            if _res[0] >= self.geode_robot.0
                && _res[2] >= self.geode_robot.1
                && _robots[0] >= self.geode_robot.0
                && _robots[2] >= self.geode_robot.1
            {
                return max_geodes;
            }
        } else if res[0] >= self.obsidian_robot.0
            && res[1] >= self.obsidian_robot.1
            && robots[2] < self.geode_robot.1
        {
            let mut _res = res;
            _res[0] -= self.obsidian_robot.0;
            _res[1] -= self.obsidian_robot.1;

            let _res = update_res(_res, robots);

            let mut _robots = robots;
            _robots[2] += 1;
            max_geodes = max(
                max_geodes,
                self.max_geodes_recursive(_res, _robots, min + 1, maxmin),
            );
        } else if res[0] >= self.clay_robot && robots[1] < self.obsidian_robot.1 {
            let mut _res = res;
            _res[0] -= self.clay_robot;

            let _res = update_res(_res, robots);

            let mut _robots = robots;
            _robots[1] += 1;
            max_geodes = max(
                max_geodes,
                self.max_geodes_recursive(_res, _robots, min + 1, maxmin),
            );
        }

        if res[0] >= self.ore_robot
            && robots[0]
                < max(
                    self.clay_robot,
                    max(self.obsidian_robot.0, self.geode_robot.0),
                )
        {
            let mut _res = res;
            _res[0] -= self.ore_robot;

            let _res = update_res(_res, robots);

            let mut _robots = robots;
            _robots[0] += 1;

            max_geodes = max(
                max_geodes,
                self.max_geodes_recursive(_res, _robots, min + 1, maxmin),
            );
        }

        let _res = update_res(res, robots);
        max_geodes = max(
            max_geodes,
            self.max_geodes_recursive(_res, robots, min + 1, maxmin),
        );

        max_geodes
    }
}

fn update_res(mut res: [usize; 4], robots: [usize; 4]) -> [usize; 4] {
    res[0] += robots[0];
    res[1] += robots[1];
    res[2] += robots[2];
    res[3] += robots[3];
    res
}

trait SplitIntoNums {
    type Separator;
    fn split_into_vec_usize(&self, sep: Self::Separator) -> Vec<usize>;
}

impl SplitIntoNums for str {
    type Separator = char;
    fn split_into_vec_usize(&self, sep: Self::Separator) -> Vec<usize> {
        self.split(sep)
            .filter_map(|w| w.parse::<usize>().ok())
            .collect()
    }
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':').map(|part| part.split_into_vec_usize(' '));
        let header = parts.next().unwrap();
        let body = parts.next().unwrap();

        Ok(Blueprint {
            line: s.to_string(),
            num: header[0],
            ore_robot: body[0],
            clay_robot: body[1],
            obsidian_robot: (body[2], body[3]),
            geode_robot: (body[4], body[5]),
        })
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let b = line.parse::<Blueprint>().unwrap();
            print!("*");
            stdout().flush();
            b.calc_max_geodes(24) * b.num
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .take(3)
        .map(|line| {
            let b = line.parse::<Blueprint>().unwrap();
            print!("*");
            stdout().flush();
            b.calc_max_geodes(32)
        })
        .product()
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn day19_part1() {
        assert_eq!(part1(TEST_INPUT), 33);
    }

    #[test]
    fn day19_part2() {
        assert_eq!(part2(TEST_INPUT), 56 * 62);
    }
}
