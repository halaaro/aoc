#![allow(unused)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::{repeat, Repeat},
    str::FromStr,
};

fn main() {
    let input = include_str!("../../data/day22.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input, 50));
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

fn part2(input: &str, face_size: usize) -> usize {
    let maze = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut s = l.to_string();
            s.extend(repeat(' ').take(150 - l.len()));
            s
        })
        .collect::<Vec<_>>();

    let cube_map = fold_cube(&maze, face_size);

    let mut map = &mut cube_map.map.iter().clone().collect::<Vec<_>>();
    map.sort();
    // dbg!(map);

    let instruction = input
        .lines()
        .last()
        .unwrap()
        .parse::<Instructions>()
        .unwrap();
    let mut pos = get_start_pos(&maze);

    for ins in instruction.0 {
        pos = make_move_cube(&maze, &cube_map, pos, ins);
    }

    (pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + pos.2 as usize
}

fn fold_cube(maze: &[impl AsRef<str>], face_size: usize) -> CubeMap {
    let mut faces: Vec<(usize, usize)> = maze
        .iter()
        .step_by(face_size)
        .enumerate()
        .flat_map(|(idx, row)| {
            row.as_ref()
                .chars()
                .step_by(face_size)
                .enumerate()
                .filter_map(move |(idx2, c)| match c {
                    '.' | '#' => Some((idx2, idx)),
                    _ => None,
                })
        })
        .collect();

    let map_size = (
        faces.iter().map(|f| f.0).max().unwrap() + 1,
        faces.iter().map(|f| f.1).max().unwrap() + 1,
    );

    let face0 = faces[0]; // basis for cube transformation
    let map = faces
        .iter()
        .map(|f| {
            let mut paths = VecDeque::new();
            let mut path = None;
            paths.push_back((face0, V3(1, 0, 0), V3(0, -1, 0)));
            let mut visited = HashSet::new();
            while let Some((iface, i, j)) = paths.pop_front() {
                if iface == *f {
                    // found it!
                    path = Some((i, j, i.cross(j)));
                    break;
                }

                if visited.contains(&iface) {
                    continue;
                }
                visited.insert(iface);

                if iface.0 > 0 && faces.contains(&(iface.0 - 1, iface.1)) {
                    paths.push_back(((iface.0 - 1, iface.1), i.cross(j), j));
                }

                if iface.0 < map_size.0 && faces.contains(&(iface.0 + 1, iface.1)) {
                    paths.push_back(((iface.0 + 1, iface.1), i.cross(-j), j));
                }

                if iface.1 > 0 && faces.contains(&(iface.0, iface.1 - 1)) {
                    paths.push_back(((iface.0, iface.1 - 1), i, j.cross(-i)));
                }

                if iface.1 < map_size.1 && faces.contains(&(iface.0, iface.1 + 1)) {
                    paths.push_back(((iface.0, iface.1 + 1), i, j.cross(i)));
                }
            }
            (*f, path.unwrap())
        })
        .collect::<HashMap<_, _>>();

    CubeMap {
        face_size,
        map,
        map_size,
    }
}

fn make_move(maze: &[impl AsRef<str>], pos: Pos, ins: Instruction) -> Pos {
    match ins {
        Left => turn(pos, -1),
        Right => turn(pos, 1),
        Forward(amt) => move_forward(maze, pos, amt),
    }
}

fn make_move_cube(maze: &[impl AsRef<str>], cube_map: &CubeMap, pos: Pos, ins: Instruction) -> Pos {
    match ins {
        Left => turn(pos, -1),
        Right => turn(pos, 1),
        Forward(amt) => move_forward_cube(maze, cube_map, pos, amt),
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

fn move_forward_cube(
    maze: &[impl AsRef<str>],
    cube_map: &CubeMap,

    mut pos: Pos,
    mut amt: usize,
) -> Pos {
    let mut dir: (i32, i32) = match pos.2 {
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

    for _ in 0..amt {
        let ((col, row), new_dir) = cube_map.next_pos(&pos, dir);
        dir = new_dir;
        next_row = ((row + rowlen) % rowlen) as usize;
        next_col = ((col + collen) % collen) as usize;

        let row = maze[next_row].as_ref();
        let next_tile = row.as_bytes()[next_col];

        if next_tile == b'#' {
            break;
        } else if next_tile == b'.' {
            pos = Pos(next_col, next_row, direction(dir));
        } else if next_tile != b' ' {
            panic!("that's not cool! I found a '{}'", next_tile);
        }
    }

    pos
}

fn direction(dir: (i32, i32)) -> usize {
    match dir {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        x => unreachable!("dir ({} {}) is inconceivable!", x.0, x.1),
    }
}

fn turn(pos: Pos, amt: i32) -> Pos {
    let newdir = (pos.2 as i32 + amt + 4) % 4;
    Pos(pos.0, pos.1, newdir as usize)
}

fn get_start_pos(maze: &[impl AsRef<str>]) -> Pos {
    Pos(maze[0].as_ref().find('.').unwrap(), 0, 0)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct V3(i32, i32, i32);

impl std::ops::Div<i32> for V3 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        V3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl std::ops::Add<V3> for V3 {
    type Output = Self;

    fn add(self, rhs: V3) -> Self::Output {
        V3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl V3 {
    pub(crate) fn cross(self, other: V3) -> V3 {
        V3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub(crate) fn dot(self, other: V3) -> i32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

impl std::ops::Neg for V3 {
    type Output = V3;

    fn neg(self) -> Self::Output {
        V3(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Mul<V3> for i32 {
    type Output = V3;
    fn mul(self, other: V3) -> Self::Output {
        V3(other.0 * self, other.1 * self, other.2 * self)
    }
}

#[derive(Debug)]
struct CubeMap {
    face_size: usize,
    map: HashMap<(usize, usize), (V3, V3, V3)>,
    map_size: (usize, usize),
}

impl CubeMap {
    fn next_pos(&self, pos: &Pos, dir: (i32, i32)) -> ((i32, i32), (i32, i32)) {
        let size = self.face_size;
        let face = (pos.0 / size, pos.1 / size);
        let inext_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        let inext_face = (inext_pos.0 / size as i32, inext_pos.1 / size as i32);

        if (face.0 as i32, face.1 as i32) == inext_face {
            return (inext_pos, dir); // same face
        }

        // get cube face basis vectors
        let (i, j, k) = *self.map.get(&face).unwrap();
        // println!("i:{i:?}, j:{j:?}, k:{k:?}");

        // convert dir to cube coords
        let k2 = dir.0 * i + dir.1 * j;

        // dbg!(dir, k2);

        let (next_face, (i2, j2, _)) = self.map.iter().find(|(k, v)| v.2 == k2).unwrap();
        let (i2, j2) = (*i2, *j2);

        //dbg!((next_face, (i2, j2, k2)));
        //dbg!(next_face);
        // println!("dir: {dir:?}");
        // println!("pos: {pos:?}");
        // println!("face: {face:?}");
        // println!("next_face: {next_face:?}");

        let face_size = self.face_size as i32;
        let sizev = V3(face_size, face_size, face_size);

        let fpos2 = (
            (face.0 * self.face_size) as i32,
            (face.1 * self.face_size) as i32,
        );

        let pos3 = ((self.face_size - 1) as i32) * ((V3(1, 1, 1) + (-i) + (-j) + k) / 2)
            + (pos.0 as i32 - fpos2.0) * i
            + (pos.1 as i32 - fpos2.1) * j;
        // println!("pos3: {pos3:?}");

        let new_dir3 = -k;
        let new_dir = (new_dir3.dot(i2), new_dir3.dot(j2));

        // println!("new_dir: {new_dir:?}");

        // if (next_face.0 as i32, next_face.1 as i32) == inext_face {
        //     return (inext_pos, new_dir); // continuous on 2d
        // }

        let new_fpos2 = (
            (next_face.0 * self.face_size) as i32,
            (next_face.1 * self.face_size) as i32,
        );
        // map to cube posfpos
        let new_fpos3 =
            pos3 + -((self.face_size - 1) as i32) * ((V3(1, 1, 1) + -i2 + -j2 + k2) / 2);

        let new_pos = (
            new_fpos3.dot(i2) + new_fpos2.0,
            new_fpos3.dot(j2) + new_fpos2.1,
        );
        // println!("new_pos: {new_pos:?}");

        (new_pos, new_dir)

        // dbg!((pos.0, pos.1), dir, face, inext_pos, inext_face);
        // normalize face coords
        // let map_size = (self.map_size.0 as i32, self.map_size.1 as i32);

        // let next_face = (
        //     ((inext_face.0 + map_size.0) % map_size.0) as usize,
        //     ((inext_face.1 + map_size.1) % map_size.1) as usize,
        // );

        // if !self.map.contains_key(&next_face) {
        //     todo!("disconnected face")
        // }

        // unsafe {
        //     cnt += 1;
        //     if cnt == 2 {
        //         dbg!(&self.map, self.map.contains(&next_face));
        //         todo!("sup");
        //     }
        // }

        // find shortest path between faces

        // todo!("connected face");

        // if !path.iter().all(|i| *i == path[0]) {
        //     // if !path.len() == 1 {
        //     todo!("collapse paths into a transformation")
        // }

        // unsafe {
        //     cnt += 1;
        //     if cnt > 5 {
        //         panic!()
        //     }
        // }

        // inext_pos
    }
}

#[derive(Debug)]
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

    #[test]
    fn day22_part2() {
        assert_eq!(part2(TEST_INPUT, 4), 5031);
    }
}
