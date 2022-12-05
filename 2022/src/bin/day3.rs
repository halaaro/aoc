fn main() {
    let input = include_str!("../../data/day3.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (c1, c2) = l.split_at(l.len() / 2);
            let (b1, b2) = (c1.as_bytes(), c2.as_bytes());

            let mut types = [false; 52];
            for b in b1 {
                match b {
                    b'a'..=b'z' => {
                        types[(b - b'a') as usize] = true;
                    }
                    b'A'..=b'Z' => {
                        types[(b - b'A' + 26) as usize] = true;
                    }
                    _ => todo!(),
                }
            }
            for b in b2 {
                match b {
                    b'a'..=b'z' if types[(b - b'a') as usize] => return b - b'a',
                    b'A'..=b'Z' if types[(b - b'A' + 26) as usize] => return b - b'A' + 26,
                    _ => (),
                }
            }
            unreachable!()
        })
        .map(|v| (v + 1) as u32)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|g| {
            let mut types = [[false; 52]; 3];
            for i in 0..3 {
                for b in g[i].bytes() {
                    match b {
                        b'a'..=b'z' => types[i][(b - b'a') as usize] = true,
                        b'A'..=b'Z' => types[i][(b - b'A' + 26) as usize] = true,
                        _ => todo!(),
                    }
                }
            }

            for i in 0..52 {
                if types[0][i] && types[1][i] && types[2][i] {
                    return i as u32 + 1;
                }
            }
            unreachable!("bad input?")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    #[test]
    fn day3_part1() {
        assert_eq!(part1(TEST_INPUT), 157);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(TEST_INPUT), 70);
    }
}
