use std::collections::HashSet;

fn main() {
    let input = include_str!("../../data/day9.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct V2(i32, i32);

impl std::ops::Add for V2 {
    type Output = V2;

    fn add(self, other: Self) -> Self::Output {
        V2(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for V2 {
    type Output = V2;

    fn sub(self, other: Self) -> Self::Output {
        V2(self.0 - other.0, self.1 - other.1)
    }
}

#[derive(Debug, Default)]
struct Sim {
    tail_moves: HashSet<V2>,
    tail: V2,
    head: V2,
}

impl Sim {
    fn new() -> Self {
        Self::default()
    }

    fn step(&mut self, dir: V2, count: i32) {
        for _ in 0..count {
            self.head = self.head + dir;
            self.update_tail();
        }
    }

    fn update_tail(&mut self) {
        let mut dist = self.head - self.tail;

        if dist.0.abs() > 1 || dist.1.abs() > 1 {
            // move
            if dist.0 != 0 {
                dist.0 /= dist.0.abs()
            }
            if dist.1 != 0 {
                dist.1 /= dist.1.abs()
            }

            self.tail = self.tail + dist;
        }

        if !self.tail_moves.contains(&self.tail) {
            self.tail_moves.insert(self.tail);
        }
    }
}

#[derive(Debug, Default)]
struct Sim2 {
    tail_moves: HashSet<V2>,
    knots: [V2; 10],
}

impl Sim2 {
    fn new() -> Self {
        Self::default()
    }

    fn step(&mut self, dir: V2, count: i32) {
        for _ in 0..count {
            self.knots[0] = self.knots[0] + dir;
            self.update_knots();
        }
    }

    fn update_knots(&mut self) {
        for i in 1..self.knots.len() {
            let mut dist = self.knots[i - 1] - self.knots[i];

            if dist.0.abs() > 1 || dist.1.abs() > 1 {
                // move
                if dist.0 != 0 {
                    dist.0 /= dist.0.abs()
                }
                if dist.1 != 0 {
                    dist.1 /= dist.1.abs()
                }

                self.knots[i] = self.knots[i] + dist;
            }

            if i == 9 && !self.tail_moves.contains(&self.knots[9]) {
                self.tail_moves.insert(self.knots[9]);
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut sim = Sim::new();

    input.lines().for_each(|l| {
        let (dir, count) = l.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();
        let dir = match dir {
            "U" => V2(0, 1),
            "L" => V2(-1, 0),
            "R" => V2(1, 0),
            "D" => V2(0, -1),
            _ => unreachable!(),
        };
        sim.step(dir, count);
    });

    sim.tail_moves.len()
}

fn part2(input: &str) -> usize {
    let mut sim = Sim2::new();

    input.lines().for_each(|l| {
        let (dir, count) = l.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();
        let dir = match dir {
            "U" => V2(0, 1),
            "L" => V2(-1, 0),
            "R" => V2(1, 0),
            "D" => V2(0, -1),
            _ => unreachable!(),
        };
        sim.step(dir, count);
    });

    sim.tail_moves.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn day9_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    const TEST_INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    #[test]
    fn day9_part2() {
        assert_eq!(part2(TEST_INPUT2), 36);
    }
}
