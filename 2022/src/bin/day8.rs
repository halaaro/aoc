use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::repeat,
};

fn main() {
    let input = include_str!("../../data/day8.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let data = input
        .lines()
        .map(|l| l.bytes().map(|b| (b - 48) as i8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let imax = data.len() as i32 - 1;
    let jmax = data[0].len() as i32 - 1;

    // dfs to find all trees starting at the edges
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let jiter = 0..=jmax;
    let iiter = 0..=imax;
    // start from each side
    for j in jiter.clone() {
        // top
        queue.push_back((0, j, (1, 0), -1));
    }
    for j in jiter {
        // bottom
        queue.push_back((imax, j, (-1, 0), -1));
    }
    for i in iiter.clone() {
        // left
        queue.push_back((i, 0, (0, 1), -1));
    }
    for i in iiter {
        // right
        queue.push_back((i, jmax, (0, -1), -1));
    }

    let mut count = 0;

    while let Some((i, j, dir, max)) = queue.pop_front() {
        let val = data[i as usize][j as usize];

        // println!("i={i}, j={j}, val={val}, dir={dir:?}, max={max}");
        if !visited.contains(&(i, j)) && val > max {
            count += 1;
            // println!("count={count}");
            visited.insert((i, j));
        }

        let max = val.max(max);
        let (i2, j2) = (i + dir.0, j + dir.1);
        if (0..=imax).contains(&i2) && (0..=jmax).contains(&j2) {
            queue.push_front((i2, j2, dir, max));
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let data = input
        .lines()
        .map(|l| l.bytes().map(|b| (b - 48) as i8).enumerate())
        .enumerate()
        .flat_map(|(i, line)| line.map(move |(j, d)| ((i as i32, j as i32), d)))
        .collect::<HashMap<_, _>>();

    let imax = input.lines().count() as i32 - 1;
    let jmax = input.lines().next().unwrap().bytes().len() as i32 - 1;
    data.iter()
        .map(|((i, j), _)| scenic_score(&data, (*i, *j), (imax, jmax)))
        .max()
        .unwrap()
}

fn scenic_score(data: &HashMap<(i32, i32), i8>, pos: (i32, i32), size: (i32, i32)) -> usize {
    let (i, j) = pos;
    let height0 = data[&(i, j)];
    let (imax, jmax) = size;
    [
        (0..=i - 1).rev().zip(repeat(j)).collect::<Vec<_>>(),
        (i + 1..=imax).zip(repeat(j)).collect::<Vec<_>>(),
        repeat(i).zip((0..=j - 1).rev()).collect::<Vec<_>>(),
        repeat(i).zip(j + 1..=jmax).collect::<Vec<_>>(),
    ]
    .into_iter()
    .map(|p| {
        let mut end_next = false;
        // println!("count={cnt}");
        p.into_iter()
            .filter(|pos| data.contains_key(pos))
            .take_while(|pos| {
                let ended = end_next;
                end_next = stop(data, height0, *pos);
                !ended
            })
            .count()
    })
    .product()
}

fn stop(data: &HashMap<(i32, i32), i8>, height0: i8, pos: (i32, i32)) -> bool {
    match data.get(&pos) {
        Some(&h) => h >= height0,
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn day8_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn day8_part2() {
        assert_eq!(part2(TEST_INPUT), 8);
    }
}
