use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../data/day18.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn parse(input: &str) -> [[[bool; 21]; 21]; 21] {
    let mut space = [[[false; 21]; 21]; 21];

    // mark cubes
    for line in input.lines() {
        let mut c = line.split(',').map(|c| c.parse::<usize>().unwrap());
        space[c.next().unwrap()][c.next().unwrap()][c.next().unwrap()] = true;
    }
    space
}

fn calc_surf_area(space: [[[bool; 21]; 21]; 21]) -> usize {
    // count marked cubes
    let cubes = space.iter().flatten().flatten().filter(|&&v| v).count();

    // count neighboring faces
    let mut nb = 0;
    for i in 0..20 {
        for j in 0..20 {
            for k in 0..20 {
                if space[i][j][k] && space[i + 1][j][k] {
                    nb += 1
                }
                if space[i][j][k] && space[i][j + 1][k] {
                    nb += 1
                }
                if space[i][j][k] && space[i][j][k + 1] {
                    nb += 1
                }
            }
        }
    }
    cubes * 6 - nb * 2
}

fn part1(input: &str) -> usize {
    let space = parse(input);
    calc_surf_area(space)
}

fn part2(input: &str) -> usize {
    let total_area = part1(input);

    let mut space = parse(input);

    // use fill to mark exterior cubes like lava cubes (union)
    let mut queue = VecDeque::new();
    queue.push_front((0i32, 0i32, 0i32));
    while let Some((x, y, z)) = queue.pop_back() {
        if space.get(x as usize).is_none()
            || space.get(y as usize).is_none()
            || space.get(z as usize).is_none()
            || space[x as usize][y as usize][z as usize]
        {
            continue;
        }
        space[x as usize][y as usize][z as usize] = true;
        queue.push_front((x - 1, y, z));
        queue.push_front((x + 1, y, z));
        queue.push_front((x, y - 1, z));
        queue.push_front((x, y + 1, z));
        queue.push_front((x, y, z - 1));
        queue.push_front((x, y, z + 1));
    }

    // invert marked to only consider interior cubes
    space
        .iter_mut()
        .flatten()
        .flatten()
        .for_each(|pt| *pt = !*pt);

    let inner_area = calc_surf_area(space);
    total_area - inner_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn day18_part1() {
        assert_eq!(part1(TEST_INPUT), 64);
    }
    #[test]
    fn day18_part2() {
        assert_eq!(part2(TEST_INPUT), 58);
    }
}
