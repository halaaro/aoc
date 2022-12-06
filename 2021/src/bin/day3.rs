fn main() {
    let input = include_str!("../../data/day3.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut dat = [0usize; 32];
    let len = input.lines().next().unwrap().len();
    let count = input.lines().count();
    input.lines().for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                dat[i] += 1
            }
        });
    });

    let gamma = dat.iter().take(len).fold(0, |acc, &v| {
        (acc << 1) + if v > (count / 2) { 1 } else { 0 }
    });

    let epsilon = (!gamma) & ((!0) >> (std::mem::size_of::<usize>() * 8 - len));
    gamma * epsilon
}

fn part2(input: &str) -> usize {
    let len = input.lines().next().unwrap().len();

    let mut lines = input.lines().collect::<Vec<_>>();
    for i in 0..len {
        let thresh = (lines.len() - 1) / 2;
        let ones = lines
            .iter()
            .filter(|l| l.chars().nth(i).unwrap() == '1')
            .count();
        let new_lines = lines
            .into_iter()
            .filter(|l| l.chars().nth(i).unwrap() == if ones > thresh { '1' } else { '0' });
        lines = new_lines.collect::<Vec<_>>();

        // dbg!(&lines);
        if lines.len() == 1 {
            break;
        }
    }

    let o2 = usize::from_str_radix(lines[0], 2).unwrap();

    let mut lines = input.lines().collect::<Vec<_>>();
    for i in 0..len {
        let thresh = (lines.len() + 1) / 2;
        let ones = lines
            .iter()
            .filter(|l| l.chars().nth(i).unwrap() == '1')
            .count();
        let new_lines = lines
            .into_iter()
            .filter(|l| l.chars().nth(i).unwrap() == if ones < thresh { '1' } else { '0' });
        lines = new_lines.collect::<Vec<_>>();

        if lines.len() == 1 {
            break;
        }
    }
    let co2 = usize::from_str_radix(lines[0], 2).unwrap();

    o2 * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";
    #[test]
    fn day3_part1() {
        assert_eq!(part1(TEST_INPUT), 198);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(TEST_INPUT), 230);
    }
}
