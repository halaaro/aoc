use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, str::FromStr};

fn main() {
    let input = include_str!("../../data/day13.input");
    println!("part1: {}", part1(input).1);
    println!("part2: {}", part2(input));
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
enum Value {
    Number(i64),
    Array(Vec<Value>),
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0.partial_cmp(r0),
            (Self::Array(l0), Self::Array(r0)) => {
                for (li, ri) in l0.iter().zip(r0.iter()) {
                    if li != ri {
                        let res = li.partial_cmp(ri);
                        if res != Some(Ordering::Equal) {
                            return res;
                        }
                    }
                }
                l0.len().partial_cmp(&r0.len())
            }
            (Self::Number(l0), Self::Array(r0)) => {
                Self::Array(vec![Self::Number(*l0)]).partial_cmp(&Self::Array(r0.clone()))
            }
            (Self::Array(l0), Self::Number(r0)) => {
                Self::Array(l0.clone()).partial_cmp(&Self::Array(vec![Self::Number(*r0)]))
            }
        }
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Value {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = serde_json::from_str(s);
        if let Err(e) = &result {
            eprintln!("Err: {} on input \"{}\"", e, s);
        }

        result
    }
}

fn part1(input: &str) -> (Vec<i64>, i64) {
    let pairs = input
        .lines()
        .step_by(3)
        .zip(input.lines().skip(1).step_by(3))
        .map(|(l, r)| (l.parse::<Value>().unwrap(), r.parse::<Value>().unwrap()));

    let mut ok_pairs = vec![];
    for (i, (left, right)) in pairs.enumerate() {
        if left < right {
            ok_pairs.push(i as i64 + 1);
        }
    }

    let count = ok_pairs.iter().sum();
    (ok_pairs, count)
}

fn part2(input: &str) -> usize {
    let v2 = serde_json::from_str::<Value>("[[2]]").unwrap();
    let v6 = serde_json::from_str::<Value>("[[6]]").unwrap();
    let mut values = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Value>().unwrap())
        .chain([v2.clone(), v6.clone()])
        .collect::<Vec<Value>>();

    values.sort();

    // for v in values.iter() {
    //     println!("{}", serde_json::to_string(&v).unwrap());
    // }

    (values.iter().position(|v| v == &v2).unwrap() + 1)
        * (values.iter().position(|v| v == &v6).unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), (vec![1, 2, 4, 6], 13));
    }

    #[test]
    fn test_ord() {
        let v1: Value = serde_json::from_str("[1,[2]]").unwrap();
        let v2: Value = serde_json::from_str("[[1],4]").unwrap();
        assert!(v1 < v2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 140);
    }
}
