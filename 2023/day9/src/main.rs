fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut nums: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            for j in 0.. {
                for i in 1..nums.len() - j {
                    nums[i - 1] = nums[i] - nums[i - 1];
                }
                if nums[..nums.len() - 1 - j].iter().all(|i| *i == 0) {
                    break;
                }
            }
            nums.into_iter().skip_while(|n| *n == 0).sum::<i32>()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut nums: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .rev() // <-- only difference!
                .collect();
            for j in 0.. {
                for i in 1..nums.len() - j {
                    nums[i - 1] = nums[i] - nums[i - 1];
                }
                if nums[..nums.len() - 1 - j].iter().all(|i| *i == 0) {
                    break;
                }
            }
            nums.into_iter().skip_while(|n| *n == 0).sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example1.txt");
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../example1.txt");
        assert_eq!(part2(input), 2);
    }
}
