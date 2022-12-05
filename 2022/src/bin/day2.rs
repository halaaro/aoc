#[derive(Debug, PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
use Hand::*;

impl Hand {
    fn points(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn win(their: &Hand, our: &Hand) -> bool {
    matches!(
        (our, their),
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper)
    )
}

fn to_elf_hand(s: &str) -> Hand {
    match s.chars().next().unwrap() {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _ => unreachable!(),
    }
}

fn to_my_hand(s: &str) -> Hand {
    match s.chars().next().unwrap() {
        'X' => Rock,
        'Y' => Paper,
        'Z' => Scissors,
        _ => unreachable!(),
    }
}

const DRAW_PTS: u32 = 3;
const WIN_PTS: u32 = 6;

fn round_points(round: (Hand, Hand)) -> u32 {
    let mut pts: u32 = round.1.points();
    if round.0 == round.1 {
        pts += DRAW_PTS;
    }
    if win(&round.0, &round.1) {
        pts += WIN_PTS;
    }
    pts
}

fn pick_lose(other: &Hand) -> Hand {
    match other {
        Rock => Scissors,
        Paper => Rock,
        Scissors => Paper,
    }
}

fn pick_win(other: &Hand) -> Hand {
    match other {
        Paper => Scissors,
        Scissors => Rock,
        Rock => Paper,
    }
}

fn to_my_hand2(s: &str, elf_hand: &Hand) -> Hand {
    match s {
        "X" => pick_lose(elf_hand),
        "Y" => elf_hand.clone(),
        "Z" => pick_win(elf_hand),
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut hands = l.split(' ');
            (
                to_elf_hand(hands.next().unwrap()),
                to_my_hand(hands.next().unwrap()),
            )
        })
        .map(round_points)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut hands = l.split(' ');
            let elf_hand = to_elf_hand(hands.next().unwrap());
            let my_hand = to_my_hand2(hands.next().unwrap(), &elf_hand);
            (elf_hand, my_hand)
        })
        .map(round_points)
        .sum()
}

fn main() {
    let input = include_str!("../../data/day2.input");

    let points1 = part1(input);
    println!("part1: {:?}", points1);

    let points2 = part2(input);
    println!("part2: {:?}", points2);
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn day2_part1() {
        assert_eq!(super::part1(TEST_INPUT), 15);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(super::part2(TEST_INPUT), 12);
    }
}
