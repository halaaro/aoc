use std::collections::HashSet;

fn main() {
    let input = include_str!("../../data/day6.input");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .bytes()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .find(|(_, win)| win.iter().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        .0
        + 4
}

#[allow(unused)]
fn part2(input: &str) -> usize {
    input
        .bytes()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
        .find(|(_, win)| win.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        .0
        + 14
}

#[cfg(test)]
mod test {

    use super::*;
    const TEST_INPUT: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn day6_part1() {
        assert_eq!(part1(TEST_INPUT[0]), 7);
        assert_eq!(part1(TEST_INPUT[1]), 5);
        assert_eq!(part1(TEST_INPUT[2]), 6);
        assert_eq!(part1(TEST_INPUT[3]), 10);
        assert_eq!(part1(TEST_INPUT[4]), 11);
    }

    #[test]
    fn day6_part2() {
        assert_eq!(part2(TEST_INPUT[0]), 19);
        assert_eq!(part2(TEST_INPUT[1]), 23);
        assert_eq!(part2(TEST_INPUT[2]), 23);
        assert_eq!(part2(TEST_INPUT[3]), 29);
        assert_eq!(part2(TEST_INPUT[4]), 26);
    }
}
