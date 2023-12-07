use std::cmp::Ordering;

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn card_value(c: char) -> usize {
    [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ]
    .iter()
    .position(|&ci| ci == c)
    .unwrap()
}

fn card_value2(c: char) -> usize {
    [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ]
    .iter()
    .position(|&ci| ci == c)
    .unwrap()
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Hand(&'static str);

#[derive(PartialEq, Eq, Debug, Clone)]
struct Hand2(&'static str);

impl Hand {
    fn cards_value(&self) -> usize {
        self.0
            .chars()
            .map(card_value)
            .fold(0, |acc, val| acc * 100 + val)
    }

    fn type_value(&self) -> usize {
        if self.is_five_of_a_kind() {
            return 105;
        }
        if self.is_four_of_a_kind() {
            return 104;
        }
        if self.is_full_house() {
            return 103;
        }
        if self.is_three_of_a_kind() {
            return 102;
        }
        if self.is_two_pair() {
            return 101;
        }
        if self.is_one_pair() {
            return 100;
        }
        99
    }
    fn counts(&self) -> [u8; 13] {
        let mut cnt = [0; 13];
        self.0.chars().map(card_value).for_each(|v| cnt[v] += 1);
        cnt
    }

    fn is_five_of_a_kind(&self) -> bool {
        self.counts().into_iter().max().unwrap() == 5
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.counts().into_iter().max().unwrap() == 4
    }

    fn is_full_house(&self) -> bool {
        let cnt = self.counts();
        cnt.iter().any(|i| i == &3) && cnt.iter().any(|i| i == &2)
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.counts().into_iter().max().unwrap() == 3
    }

    fn is_two_pair(&self) -> bool {
        self.counts().into_iter().filter(|c| *c == 2).count() == 2
    }

    fn is_one_pair(&self) -> bool {
        *self.counts().iter().max().unwrap() == 2
    }
}

impl Hand2 {
    fn cards_value(&self) -> usize {
        self.0
            .chars()
            .map(card_value2)
            .fold(0, |acc, val| acc * 100 + val)
    }

    fn type_value(&self) -> usize {
        let jokers = self.joker_count();
        if self.is_five_of_a_kind() {
            return 105;
        }
        if self.is_four_of_a_kind() {
            return 104;
        }
        if self.is_full_house_no_jokers() || (jokers == 1 && self.is_two_pair_no_jokers()) {
            return 103;
        }
        if self.is_three_of_a_kind_no_jokers() || (jokers == 1 && self.is_one_pair_no_jokers()) {
            return 102;
        }
        if self.is_two_pair_no_jokers() {
            return 101;
        }
        if self.is_one_pair_no_jokers() || jokers == 1 {
            return 100;
        }
        99
    }
    fn counts_no_jokers(&self) -> [u8; 13] {
        let mut cnt = [0; 13];
        self.0
            .chars()
            .filter(|c| *c != 'J')
            .map(card_value)
            .for_each(|v| cnt[v] += 1);
        cnt
    }

    fn joker_count(&self) -> u8 {
        self.0.chars().filter(|c| *c == 'J').count() as _
    }

    fn is_five_of_a_kind(&self) -> bool {
        let max_cnt = self.counts_no_jokers().into_iter().max().unwrap();
        max_cnt + self.joker_count() == 5
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.counts_no_jokers().into_iter().max().unwrap() + self.joker_count() == 4
    }

    fn is_full_house_no_jokers(&self) -> bool {
        let cnt = self.counts_no_jokers();
        cnt.iter().any(|i| i == &3) && cnt.iter().any(|i| i == &2)
    }

    fn is_three_of_a_kind_no_jokers(&self) -> bool {
        self.counts_no_jokers().into_iter().max().unwrap() + self.joker_count() == 3
    }

    fn is_two_pair_no_jokers(&self) -> bool {
        self.counts_no_jokers()
            .into_iter()
            .filter(|c| *c == 2)
            .count()
            == 2
    }

    fn is_one_pair_no_jokers(&self) -> bool {
        *self.counts_no_jokers().iter().max().unwrap() == 2
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.type_value().partial_cmp(&other.type_value()) {
            Some(Ordering::Equal) => self.cards_value().partial_cmp(&other.cards_value()),
            o => o,
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.type_value().partial_cmp(&other.type_value()) {
            Some(Ordering::Equal) => self.cards_value().partial_cmp(&other.cards_value()),
            o => o,
        }
    }
}

fn part1(input: &'static str) -> u32 {
    // parse input
    let mut hands = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bet)| (Hand(hand), bet.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    // sort hands
    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // calculate winnings
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.1 * (i as u32 + 1))
        .sum()
}

fn part2(input: &'static str) -> u32 {
    // parse input
    let mut hands = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bet)| (Hand2(hand), bet.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    // sort hands
    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // calculate winnings
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.1 * (i as u32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../example1.txt");
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../example1.txt");
        assert_eq!(part2(input), 5905);
    }

    #[test]
    fn hand2_type_value() {
        assert_eq!(Hand2("AAAAA").type_value(), 105);
        assert_eq!(Hand2("AAAAJ").type_value(), 105);
        assert_eq!(Hand2("AJJJJ").type_value(), 105);
        assert_eq!(Hand2("JJJJJ").type_value(), 105);

        assert_eq!(Hand2("AAAA9").type_value(), 104);
        assert_eq!(Hand2("AAA9J").type_value(), 104);
        assert_eq!(Hand2("AA9JJ").type_value(), 104);
        assert_eq!(Hand2("A9JJJ").type_value(), 104);

        assert_eq!(Hand2("AA999").type_value(), 103);
        assert_eq!(Hand2("AA99J").type_value(), 103);

        assert_eq!(Hand2("AT999").type_value(), 102);
        assert_eq!(Hand2("AJT99").type_value(), 102);

        assert_eq!(Hand2("AAT99").type_value(), 101);

        assert_eq!(Hand2("AA234").type_value(), 100);
        assert_eq!(Hand2("A234J").type_value(), 100);

        assert_eq!(Hand2("A2345").type_value(), 99);
    }
}
