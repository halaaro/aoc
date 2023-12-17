use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    let input = include_str!("../input.txt");
    #[cfg(feature = "unsafe_memo")]
    {
        *unsafe { &mut MEMO } = Some(Default::default());
    }
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let lines = parse(input);
    lines
        .iter()
        .filter_map(|(s, v)| count_combos(s, v, &None))
        .sum()
}

fn part2(input: &str) -> usize {
    let lines = parse(input)
        .into_iter()
        .map(|(s, v)| {
            let vlen = v.len();
            (
                String::from_iter([s, "?"].into_iter().cycle().take(9)),
                v.into_iter().cycle().take(5 * vlen).collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let memo = Some(Default::default());
    lines
        .into_iter()
        .filter_map(|(s, v)| count_combos(&s, &v, &memo))
        .sum()
}

type Args = (usize, usize);
type Memo = Rc<RefCell<HashMap<Args, Option<usize>>>>;

#[cfg(feature = "unsafe_memo")]
static mut MEMO: Option<HashMap<Args, Option<usize>>> = None;

fn count_combos(input: &str, springs: &[usize], memo: &Option<Memo>) -> Option<usize> {
    // println!();
    // dbg!(input, springs);

    #[cfg(feature = "unsafe_memo")]
    if let Some(ret) = unsafe { &MEMO }
        .as_ref()
        .and_then(|memo| memo.get(&(input.as_ptr() as _, springs.as_ptr() as _)))
    {
        return *ret;
    }

    #[cfg(not(feature = "unsafe_memo"))]
    if let Some(ret) = memo
        .as_ref()
        .and_then(|memo| memo.try_borrow().ok())
        .and_then(|memo| {
            memo.get(&(input.as_ptr() as _, springs.as_ptr() as _))
                .copied()
        })
    {
        return ret;
    }

    if springs.is_empty() {
        if !input.contains('#') {
            // println!("base case: out of springs, hooray!");
            return Some(1);
        }
        // println!("invalid: no springs left, but we still have #");
        return None;
    }
    let skip_prefix = input.chars().take_while(|c| *c == '.').count();
    let input = &input[skip_prefix..];

    if input.is_empty() {
        // println!("invalid: no more input");
        return None;
    }

    let spring_prefix = input.chars().take_while(|c| *c == '#').count();
    let spring = springs[0];
    if spring_prefix > spring || input.len() < spring {
        // println!("invalid: prefix > spring or input.len() < spring");
        return None;
    }

    let ret = Some(
        (0..=input.len() - spring)
            .filter(|i| {
                // cannot skip over any '#'
                !input.chars().take(*i).any(|c| c == '#')
                // range is valid
                && input
                    .chars()
                    .skip(*i)
                    .take_while(|c| matches!(*c, '?' | '#'))
                    .take(spring)
                    .count()
                    == spring
                // next character must be end of range
                && !matches!(input.chars().nth(spring + *i), Some('#'))
            })
            .map(|i| {
                let next = input.len().min(spring + i + 1);
                // dbg!(&input[i..i + spring]);
                &input[next..]
            })
            .filter_map(|inp| count_combos(inp, &springs[1..], memo))
            .sum(),
    );

    #[cfg(feature = "unsafe_memo")]
    if let Some(memo) = unsafe { &mut MEMO }.as_mut() {
        memo.insert((input.as_ptr() as _, springs.as_ptr() as _), ret);
    }

    #[cfg(not(feature = "unsafe_memo"))]
    if let Some(mut memo) = memo.as_ref().and_then(|memo| memo.try_borrow_mut().ok()) {
        memo.insert((input.as_ptr() as _, springs.as_ptr() as _), ret);
    }

    ret
}

fn parse(input: &str) -> std::vec::Vec<(&str, std::vec::Vec<usize>)> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(p, nums)| (p, nums.split(',').map(|n| n.parse().unwrap()).collect()))
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_part1(#[case] input: &str, #[case] expect: usize) {
        assert_eq!(part1(input), expect);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn test_part2(#[case] input: &str, #[case] expect: usize) {
        assert_eq!(part2(input), expect);
    }
}
