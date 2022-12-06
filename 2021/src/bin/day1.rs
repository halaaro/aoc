fn main() {
    let input = include_str!("../../data/day1.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let depths = input.lines().map(|l| l.parse::<i32>().unwrap());

    depths
        .clone()
        .skip(1)
        .zip(depths)
        .filter(|(d2, d1)| d2 > d1)
        .count()
}

fn part2(input: &str) -> usize {
    let depths = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    depths
        .windows(3)
        .clone()
        .skip(1)
        .zip(depths.windows(3))
        .filter(|(d2, d1)| d2.iter().sum::<i32>() > d1.iter().sum::<i32>())
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263
";
    #[test]
    fn day1_part1() {
        assert_eq!(part1(TEST_INPUT), 7);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(TEST_INPUT), 5);
    }
}
