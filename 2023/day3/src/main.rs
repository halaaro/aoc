#![allow(dead_code)]

use nom::character::complete;
use std::collections::HashMap;

use day3::GroupCollect;

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = build_grid(input);
    let numbers = get_numbers(input);

    numbers
        .into_iter()
        .filter(|&(x, y, num)| {
            let mut check_pos = (y - 1..=y + 1)
                .flat_map(|y| [(x - 1, y), (x + num.len() as i32, y)])
                .chain((x..x + num.len() as i32).flat_map(|x| [(x, y - 1), (x, y + 1)]));
            check_pos.any(|(x, y)| grid.get(&(x, y)).map(is_sym).unwrap_or(false))
        })
        .map(|(_, _, num)| num.parse::<u32>().unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    let grid = build_grid(input);
    let numbers = get_numbers(input);

    let gear_nums = numbers
        .into_iter()
        .flat_map(|(x, y, num)| {
            let check_pos = (y - 1..=y + 1)
                .flat_map(move |y| [(x - 1, y), (x + num.len() as i32, y)])
                .chain((x..x + num.len() as i32).flat_map(move |x| [(x, y - 1), (x, y + 1)]));
            let adj_gears = check_pos.filter(|(x, y)| matches!(grid.get(&(*x, *y)), Some('*')));
            adj_gears.map(move |gpos| (gpos, num))
        })
        .group_collect::<Vec<_>,Vec<_>>();

    // println!("gears_nums: {:?}", &gear_nums);

    gear_nums
        .into_iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| {
            nums.into_iter()
                .map(|num| num.parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum()
}

fn build_grid(input: &str) -> std::collections::HashMap<(i32, i32), char> {
    let (mut x, mut y) = (0, 0);
    let mut grid = HashMap::new();
    // build hashmap
    for c in input.chars() {
        match c {
            '\n' => {
                x = 0;
                y += 1
            }
            _ => {
                grid.insert((x, y), c);
                x += 1;
            }
        }
    }
    grid
}

fn get_numbers(mut input: &str) -> Vec<(i32, i32, &str)> {
    let (mut x, mut y) = (0i32, 0i32);
    let mut numbers = Vec::new();
    loop {
        if let Ok((i, num)) = complete::digit1::<_, ()>(input) {
            numbers.push((x, y, num));
            input = i;
            x += num.len() as i32;
        } else {
            match input.chars().next() {
                Some('\n') => {
                    y += 1;
                    x = 0;
                }
                Some(_) => {
                    x += 1;
                }
                None => break,
            }
            input = &input[1..];
        }
    }
    numbers
}

fn is_sym(c: &char) -> bool {
    !matches!(c, '0'..='9' | '\n' | '.')
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../example1.txt");
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../example2.txt");
        assert_eq!(part2(input), 467835);
    }
}
