fn main() {
    let input = include_str!("../../data/day10.input");
    println!("part1: {}", part1(input));
    println!("part2: ");
    part2(input);
}

enum Inst {
    Addx(i32),
    Noop,
}

impl std::str::FromStr for Inst {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        match split.next() {
            Some("addx") => Ok(Inst::Addx(split.next().unwrap().parse().unwrap())),
            Some("noop") => Ok(Inst::Noop),
            _ => Err(()),
        }
    }
}

fn part1(input: &str) -> usize {
    let mut ins = input.lines().filter_map(|l| l.parse::<Inst>().ok());

    let mut wait_cycles = 0;
    let mut x = 1;
    let mut addx = 0;

    let mut output = 0;

    for cycle in 1.. {
        if wait_cycles == 0 {
            match ins.next() {
                Some(Inst::Noop) => {
                    wait_cycles = 1;
                    // println!("noop");
                }
                Some(Inst::Addx(v)) => {
                    wait_cycles = 2;
                    // println!("addx {v}");
                    addx = v;
                }
                _ => break,
            }
        }

        if (cycle - 20) % 40 == 0 {
            let strength = x * cycle;
            // println!("strength = {strength}");
            output += strength;
        }

        // println!("{cycle}: x={x}, addx={addx}");

        // "after"
        if wait_cycles == 1 {
            x += addx;
            addx = 0;
        }
        wait_cycles -= 1;
    }

    output as usize
}

fn part2(input: &str) -> usize {
    let mut ins = input.lines().filter_map(|l| l.parse::<Inst>().ok());

    let mut wait_cycles = 0;
    let mut x = 1;
    let mut addx = 0;

    let mut buffer = String::new();

    for cycle in 1.. {
        if wait_cycles == 0 {
            match ins.next() {
                Some(Inst::Noop) => {
                    wait_cycles = 1;
                    // println!("noop");
                }
                Some(Inst::Addx(v)) => {
                    wait_cycles = 2;
                    // println!("addx {v}");
                    addx = v;
                }
                _ => break,
            }
        }

        let xpos = (cycle - 1) % 40;
        if (xpos - 1..=xpos + 1).contains(&(x % 40)) {
            buffer.push('#');
        } else {
            buffer.push('.');
        }

        if xpos == 39 {
            buffer.push('\n');
        }

        // println!("{cycle}: x={x}, addx={addx}");

        // "after"
        if wait_cycles == 1 {
            x += addx;
            addx = 0;
        }
        wait_cycles -= 1;
    }

    print!("{buffer}");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "

addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn day10_part1() {
        assert_eq!(part1(TEST_INPUT), 13140);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
