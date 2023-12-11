use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input, '7', Direction::Up));
    println!("part 2: {}", part2(input, '7', Direction::Up));
}

fn part1(input: &str, s_shape: char, initial_dir: Direction) -> usize {
    let grid = build_grid(input);
    let coords = navigate(&grid, s_shape, initial_dir);
    coords.len() / 2
}

fn part2(input: &str, s_shape: char, initial_dir: Direction) -> usize {
    let grid = build_grid(input);
    let coords = navigate(&grid, s_shape, initial_dir)
        .into_iter()
        .collect::<BTreeMap<_, _>>();
    let mut count = 0;
    for (i, line) in input.lines().enumerate() {
        let mut inside = false;
        let mut start = None;
        for (j, c) in line.chars().enumerate() {
            let c = if c == 'S' { s_shape } else { c };
            let is_path = coords.contains_key(&(i, j));
            let crossing = is_path && matches!(c, '7' | '|' | 'J');
            let entering = is_path && matches!(c, 'F' | 'L');
            if entering {
                start = Some(c);
            } else if crossing {
                match (start, c) {
                    (None, '|') => {
                        inside = !inside;
                    }
                    (Some('L'), '7') | (Some('F'), 'J') => {
                        inside = !inside;
                        start = None;
                    }
                    (Some(_), _) => {
                        start = None;
                    }
                    _ => unreachable!(),
                }
            }

            if !is_path && inside {
                count += 1;
                // print!("I");
            } else {
                // print!("{c}");
            }
        }
        // println!();
    }
    count
}

fn build_grid(input: &str) -> std::collections::BTreeMap<(usize, usize), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| ((i, j), c)))
        .collect::<BTreeMap<_, _>>()
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn navigate(
    grid: &BTreeMap<(usize, usize), char>,
    s_shape: char,
    initial_dir: Direction,
) -> Vec<((usize, usize), (char, Direction))> {
    let mut dir = initial_dir;
    let mut pos = *grid.iter().find(|&(_, val)| *val == 'S').unwrap().0;
    let mut pipe = s_shape;
    let mut path = vec![(pos, ('S', dir))];

    use Direction::*;
    while pipe != 'S' {
        (pos, dir) = match (pipe, dir) {
            ('F', Up) => ((pos.0, pos.1 + 1), Right),
            ('F', Left) => ((pos.0 + 1, pos.1), Down),
            ('J', Right) => ((pos.0 - 1, pos.1), Up),
            ('J', Down) => ((pos.0, pos.1 - 1), Left),
            ('|', Up) => ((pos.0 - 1, pos.1), Up),
            ('|', Down) => ((pos.0 + 1, pos.1), Down),
            ('-', Right) => ((pos.0, pos.1 + 1), Right),
            ('-', Left) => ((pos.0, pos.1 - 1), Left),
            ('L', Down) => ((pos.0, pos.1 + 1), Right),
            ('L', Left) => ((pos.0 - 1, pos.1), Up),
            ('7', Up) => ((pos.0, pos.1 - 1), Left),
            ('7', Right) => ((pos.0 + 1, pos.1), Down),
            _ => unreachable!("did not handle {pipe}, going {dir:?}"),
        };
        pipe = grid[&pos];
        path.push((pos, (pipe, dir)));
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(part1(input, 'F', Direction::Up), 8);
    }

    #[test]
    fn test_part2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(input, '7', Direction::Right), 10);
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part2(input, 'F', Direction::Up), 4);
    }
}
