#![allow(dead_code)]
fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let line_digits = input
        .lines()
        .map(|line| {
            (
                line.chars().find(|c| c.is_ascii_digit()).unwrap(),
                line.chars().rev().find(|c| c.is_ascii_digit()).unwrap(),
            )
        })
        .map(|d| d.0.to_digit(10).unwrap() * 10 + d.1.to_digit(10).unwrap());

    // println!("{:?}", digits.collect::<Vec<_>>());
    line_digits.sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let digits = b"one,two,three,four,five,six,seven,eight,nine"
        .split(|bi| *bi == b',')
        .collect::<Vec<_>>();
    // println!("{:?}", &digits);

    let line_digits = input
        .lines()
        .map(|line| {
            let b = line.as_bytes();
            // println!("{}", line);
            (
                b.iter()
                    .enumerate()
                    .find_map(|(i, bi)| {
                        if *bi >= b'1' && *bi <= b'9' {
                            return Some(bi - b'0');
                        }
                        digits
                            .iter()
                            .enumerate()
                            .find(|(_, dj)| b.len() - i >= dj.len() && b[i..i + dj.len()] == ***dj)
                            .map(|(j, _)| (j + 1) as u8)
                    })
                    .unwrap(),
                b.iter()
                    .enumerate()
                    .rev()
                    .find_map(|(i, bi)| {
                        if *bi >= b'1' && *bi <= b'9' {
                            return Some(bi - b'0');
                        }
                        digits
                            .iter()
                            .enumerate()
                            .find(|(_, dj)| b.len() - i >= dj.len() && b[i..i + dj.len()] == ***dj)
                            .map(|(j, _)| (j + 1) as u8)
                    })
                    .unwrap(),
            )
        })
        .map(|d| (d.0 as u32) * 10 + d.1 as u32);

    // println!("{:?}", line_digits.collect::<Vec<_>>());
    line_digits.sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example1 = include_str!("../example1.txt");
        assert_eq!(part1(example1), 142);
    }

    #[test]
    fn part2_example() {
        let example2 = include_str!("../example2.txt");
        assert_eq!(part2(example2), 281);
    }
}
