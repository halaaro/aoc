#![allow(unused)]
use std::{
    iter::{repeat, Repeat},
    str::FromStr,
};

fn main() {
    let input = include_str!("../../data/day22.input");
    println!("part1: {}", part1(input));
}
fn part1(input: &str) -> usize {
    let maze = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut s = l.to_string();
            s.extend(repeat(' ').take(150 - l.len()));
            s
        })
        .collect::<Vec<_>>();
    let instruction = input
        .lines()
        .last()
        .unwrap()
        .parse::<Instructions>()
        .unwrap();
    let mut pos = get_start_pos(&maze);

    for ins in instruction.0 {
        pos = make_move(&maze, pos, ins);
    }

    (pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + pos.2 as usize
}

fn make_move(maze: &[impl AsRef<str>], pos: Pos, ins: Instruction) -> Pos {
    match ins {
        Left => turn(pos, -1),
        Right => turn(pos, 1),
        Forward(amt) => move_forward(maze, pos, amt),
    }
}

fn move_forward(maze: &[impl AsRef<str>], mut pos: Pos, mut amt: usize) -> Pos {
    let dir: (i32, i32) = match pos.2 {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        x => unreachable!("dir {} is inconceivable!", x),
    };

    let rowlen = maze.len() as i32;
    let collen = maze[0].as_ref().len() as i32;

    let mut next_col = pos.0;
    let mut next_row = pos.1;

    while amt > 0 {
        next_row = ((next_row as i32 + dir.1 + rowlen) % rowlen) as usize;
        next_col = ((next_col as i32 + dir.0 + collen) % collen) as usize;
        let row = maze[next_row].as_ref();
        let next_tile = row.as_bytes()[next_col];

        if next_tile == b'#' {
            break;
        } else if next_tile == b'.' {
            pos = Pos(next_col, next_row, pos.2);
            amt -= 1;
        } else if next_tile != b' ' {
            panic!("that's not cool! I found a '{}'", next_tile);
        }
    }

    pos
}

fn turn(pos: Pos, amt: i32) -> Pos {
    let newdir = (pos.2 as i32 + amt + 4) % 4;
    Pos(pos.0, pos.1, newdir as usize)
}

fn get_start_pos(maze: &[impl AsRef<str>]) -> Pos {
    Pos(maze[0].as_ref().find('.').unwrap(), 0, 0)
}

struct Pos(usize, usize, usize);

#[derive(Debug, PartialEq)]
enum Instruction {
    Forward(usize),
    Right,
    Left,
}
use Instruction::*;

#[derive(Debug, PartialEq)]
struct Instructions(Vec<Instruction>);

impl FromStr for Instructions {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = String::new();
        let mut res = vec![];
        for c in s.chars() {
            if !c.is_numeric() && !buf.is_empty() {
                res.push(Forward(buf.parse::<usize>()?));
                buf.clear();
            }
            match c {
                'L' => res.push(Left),
                'R' => res.push(Right),
                c => buf.push(c),
            }
        }

        if !buf.is_empty() {
            res.push(Forward(buf.parse::<usize>()?));
        }

        Ok(Instructions(res))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";
    #[test]
    fn parse_instructions_test() {
        let input = "10R5";
        assert_eq!(
            input.parse(),
            Ok(Instructions(vec![Forward(10), Right, Forward(5)]))
        );
    }

    #[test]
    fn day22_part1() {
        assert_eq!(part1(TEST_INPUT), 6032);
    }
}
