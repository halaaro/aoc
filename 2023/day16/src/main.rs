use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input, (0, 0), R));
    println!("part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    L,
    R,
    U,
    D,
}
use Dir::*;

fn part1(input: &str, start_pos: (i32, i32), start_dir: Dir) -> usize {
    let grid = parse(input);
    count_visited(&grid, start_pos, start_dir)
}

fn count_visited(grid: &HashMap<(i32, i32), char>, start_pos: (i32, i32), start_dir: Dir) -> usize {
    let mut visited = HashMap::<_, Vec<Dir>>::new();
    let mut p = vec![(start_pos, start_dir)];
    while let Some(((i, j), dir)) = p.pop() {
        let g = grid.get(&(i, j));
        if visited
            .get(&(i, j))
            .map(|v| v.contains(&dir))
            .unwrap_or_default()
            || g.is_none()
        {
            continue;
        }
        visited.entry((i, j)).or_default().push(dir);
        match (g, dir) {
            (Some('\\'), R) | (Some('/'), L) | (Some('|' | '.'), D) => p.push(((i + 1, j), D)),
            (Some('/'), R) | (Some('\\'), L) | (Some('|' | '.'), U) => p.push(((i - 1, j), U)),
            (Some('\\'), U) | (Some('/'), D) | (Some('-' | '.'), L) => p.push(((i, j - 1), L)),
            (Some('/'), U) | (Some('\\'), D) | (Some('-' | '.'), R) => p.push(((i, j + 1), R)),
            (Some('|'), L | R) => p.extend([((i + 1, j), D), ((i - 1, j), U)]),
            (Some('-'), U | D) => p.extend([((i, j + 1), R), ((i, j - 1), L)]),
            other => todo!("{other:?}"),
        }
        // println!();
        // for i in 0..10 {
        //     let row = (0..10)
        //         .filter_map(|j| {
        //             visited
        //                 .get(&(i, j))
        //                 .map(|v| match v.len() {
        //                     2 => &'2',
        //                     3 => &'3',
        //                     4 => &'4',
        //                     _ => &'#',
        //                 })
        //                 .or_else(|| grid.get(&(i, j)))
        //         })
        //         .collect::<String>();
        //     println!("{row}");
        // }
    }
    visited.values().filter(|v| !v.is_empty()).count()
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    let (max_i, max_j) = grid
        .keys()
        .fold((0, 0), |acc, c| (c.0.max(acc.0), c.1.max(acc.1)));
    let starts = (0..=max_j)
        .flat_map(|j| [(0, j, D), (max_i, j, U)])
        .chain((0..=max_i).flat_map(|i| [(i, 0, R), (i, max_j, L)]));
    starts
        .map(|(i, j, dir)| count_visited(&grid, (i, j), dir))
        .max()
        .unwrap()
}

fn parse(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example1.txt");
        assert_eq!(part1(input, (0, 0), R), 46);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../example1.txt");
        assert_eq!(part2(input), 51);
    }
}
