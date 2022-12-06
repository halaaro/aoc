use std::str::FromStr;

fn main() {
    let input = include_str!("../../data/day2.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl Direction {
    fn horiz(&self) -> i32 {
        match self {
            Forward => 1,
            _ => 0,
        }
    }
    fn vert(&self) -> i32 {
        match self {
            Up => -1,
            Down => 1,
            _ => 0,
        }
    }
}

use Direction::*;

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Forward),
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(()),
        }
    }
}

fn part1(input: &str) -> i32 {
    let pos = input
        .lines()
        .map(|l| {
            let (dir, mag) = l.split_once(' ').unwrap();
            (
                dir.parse::<Direction>().unwrap(),
                mag.parse::<i32>().unwrap(),
            )
        })
        .fold((0, 0), |(x, y), (d, m)| {
            (x + d.horiz() * m, y + d.vert() * m)
        });
    pos.0 * pos.1
}

fn part2(input: &str) -> i32 {
    let pos = input
        .lines()
        .map(|l| {
            let (dir, mag) = l.split_once(' ').unwrap();
            (
                dir.parse::<Direction>().unwrap(),
                mag.parse::<i32>().unwrap(),
            )
        })
        .fold((0, 0, 0), |(x, y, a), (d, m)| {
            (x + d.horiz() * m, y + d.horiz() * m * a, a + d.vert() * m)
        });
    pos.0 * pos.1
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn day2_part1() {
        assert_eq!(part1(TEST_INPUT), 150);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(part2(TEST_INPUT), 900);
    }
}
