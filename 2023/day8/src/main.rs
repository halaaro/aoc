use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../input.txt");

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn parse(input: &'static str) -> (Vec<usize>, BTreeMap<&'static str, [&'static str; 2]>) {
    let dirs = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<_>>();
    let network = input
        .lines()
        .skip(2)
        .map(|line| {
            let (src, lr) = line.split_once(" = (").unwrap();
            let (left, right) = lr[..8].split_once(", ").unwrap();
            (src, [left, right])
        })
        .collect::<BTreeMap<_, _>>();
    (dirs, network)
}

fn part1(input: &'static str) -> usize {
    let (dirs, network) = parse(input);
    let dir_len = dirs.len();
    let mut el = "AAA";
    for i in 0.. {
        el = network[el][dirs[i % dir_len]];
        if el == "ZZZ" {
            return 1 + i;
        }
    }
    0
}

fn part2(input: &'static str) -> usize {
    let (dirs, network) = parse(input);
    let dir_len = dirs.len();

    let pos = network
        .keys()
        .filter(|k| &k[2..] == "A")
        .copied()
        .collect::<Vec<_>>();

    pos.into_iter()
        .map(|mut p| {
            for i in 0.. {
                p = network[&p][dirs[i % dir_len]];
                if p.ends_with('Z') {
                    return i + 1;
                }
            }
            0
        })
        .reduce(lcm)
        .unwrap()
}

fn lcm(mut a: usize, b: usize) -> usize {
    a /= gcd(a, b);
    a * b
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    };
    while b > 0 {
        if a > b {
            std::mem::swap(&mut a, &mut b)
        };
        b %= a;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input), 6);
    }
}
