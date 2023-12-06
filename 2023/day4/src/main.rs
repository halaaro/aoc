use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {:?}", part1(input));
    println!("part2: {:?}", part2(input));
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn part1(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (card, nums) = line.split_once(':').ok_or("no :")?;
        let (_, _card) = card.split_once(' ').ok_or("no space after card")?;

        let (winners, numbers) = nums.split_once('|').ok_or("no |")?;
        let winners = winners.trim().split_ascii_whitespace().collect::<Vec<_>>();
        let numbers = numbers.trim().split_ascii_whitespace();
        let wins = numbers.filter(|n| winners.contains(n)).count();
        if wins > 0 {
            sum += 1 << (wins - 1)
        }
    }

    Ok(sum)
}
fn part2(input: &str) -> Result<u32> {
    let mut sum = 0;
    let mut copies = VecDeque::new();
    for line in input.lines() {
        let (card, nums) = line.split_once(':').ok_or("no :")?;
        let (_, _card) = card.split_once(' ').ok_or("no space after card")?;

        let (winners, numbers) = nums.split_once('|').ok_or("no |")?;
        let winners = winners.trim().split_ascii_whitespace().collect::<Vec<_>>();
        let numbers = numbers.trim().split_ascii_whitespace();
        let score = numbers.filter(|n| winners.contains(n)).count();
        let card_count = 1 + copies.pop_front().unwrap_or(0);
        (0..score).for_each(|sc| {
            if sc >= copies.len() {
                copies.push_back(card_count)
            } else {
                copies[sc] += card_count
            }
        });
        // dbg!(&copies);
        sum += card_count;
    }

    Ok(sum as u32)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example1.txt");
        assert_eq!(part1(input).ok(), Some(13));
    }
    #[test]
    fn test_part2() {
        let input = include_str!("../example1.txt");
        assert_eq!(part2(input).ok(), Some(30));
    }
}
