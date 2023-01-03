#![allow(unused)]

fn main() {
    let input = include_str!("../../data/day11.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[derive(Default, Debug, PartialEq)]
enum Op {
    #[default]
    Unknown,
    Add(i64),
    Mult(i64),
    Square,
}

#[derive(Default)]
struct Monkey {
    num: usize,
    items: Vec<i64>,
    op: Op,
    test_div: i64,
    throw: (usize, usize),
    count_inspected: usize,
}

impl Monkey {
    fn do_throw(&self, item: i64, worry_reductor: i64, worry_reductor2: i64) -> (usize, i64) {
        let mut item = match &self.op {
            Op::Square => item * item,
            Op::Add(v) => item + v,
            Op::Mult(v) => item * v,
            Op::Unknown => todo!(),
        };

        // bored
        item /= worry_reductor;

        // other
        item %= worry_reductor2;

        let next_monkey = if item % self.test_div == 0 {
            self.throw.0
        } else {
            self.throw.1
        };

        (next_monkey, item)
    }
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .chunks(7)
        .map(|c| {
            let num = c[0]
                .split_once(' ')
                .unwrap()
                .1
                .trim_end_matches(':')
                .parse::<usize>()
                .unwrap();
            let items = c[1]
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect();
            let op_str = c[2].split_once("new = old ").unwrap().1;
            let op = match op_str.chars().next() {
                Some('*') => {
                    if op_str == "* old" {
                        Op::Square
                    } else {
                        Op::Mult(op_str[2..].parse().unwrap())
                    }
                }
                Some('+') => Op::Add(op_str[2..].parse().unwrap()),
                _ => Op::Unknown,
            };
            let test_div = c[3].split_once("divisible by ").unwrap().1.parse().unwrap();
            let throw = (
                c[4].rsplit_once(' ').unwrap().1.parse().unwrap(),
                c[5].rsplit_once(' ').unwrap().1.parse().unwrap(),
            );

            Monkey {
                num,
                items,
                op,
                test_div,
                throw,
                ..Monkey::default()
            }
        })
        .collect()
}

fn do_rounds(input: &str, rounds: usize, worry_reductor: i64, mod_reductor: bool) -> usize {
    let mut monkeys = get_monkeys(input);
    let num_monkeys = monkeys.len();

    let worry_reductor2 = if mod_reductor {
        monkeys.iter().map(|m| m.test_div).product()
    } else {
        i64::MAX
    };

    for _ in 0..rounds {
        for m in 0..num_monkeys {
            let throws = monkeys[m]
                .items
                .iter()
                .map(|i| monkeys[m].do_throw(*i, worry_reductor, worry_reductor2))
                .collect::<Vec<_>>();
            monkeys[m].count_inspected += throws.len();
            monkeys[m].items.clear();
            for throw in throws {
                monkeys[throw.0].items.push(throw.1);
            }
        }
    }
    let mut counts: Vec<_> = monkeys.into_iter().map(|m| m.count_inspected).collect();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

fn part1(input: &str) -> usize {
    do_rounds(input, 20, 3, false)
}

fn part2(input: &str) -> usize {
    do_rounds(input, 10_000, 1, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_build_monkey() {
        let monkeys = get_monkeys(TEST_INPUT);
        assert_eq!(monkeys.len(), 4);

        assert_eq!(monkeys[0].num, 0);
        assert_eq!(monkeys[1].num, 1);

        assert_eq!(monkeys[0].items, vec![79, 98]);

        assert_eq!(monkeys[0].op, Op::Mult(19));
        assert_eq!(monkeys[1].op, Op::Add(6));
        assert_eq!(monkeys[2].op, Op::Square);

        assert_eq!(monkeys[0].test_div, 23);
        assert_eq!(monkeys[0].throw, (2, 3));
    }

    #[test]
    fn test_day11_part1() {
        assert_eq!(part1(TEST_INPUT), 10605);
    }

    #[test]
    fn test_day11_part2() {
        assert_eq!(part2(TEST_INPUT), 2713310158);
    }
}
