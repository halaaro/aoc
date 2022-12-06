fn main() {
    let input = include_str!("../../data/day5.input");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut lines = input.lines().peekable();
    let mut stacks = vec![];
    let stack_width = lines.peek().unwrap().len() / 4 + 1;
    for _ in 0..stack_width {
        stacks.push(vec![])
    }
    for line in lines.by_ref() {
        if line.starts_with(" 1 ") {
            break;
        }
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(idx, c)| {
                if c != ' ' {
                    stacks[idx].insert(0, c);
                }
            });
    }

    assert_eq!(lines.next().unwrap(), "");

    let instructions = lines.map(|l| {
        l.split(' ')
            .skip(1)
            .step_by(2)
            .map(|v| v.parse::<usize>().unwrap())
    });
    for mut ins in instructions {
        let count = ins.next().unwrap();
        let source = ins.next().unwrap() - 1;
        let target = ins.next().unwrap() - 1;
        for _ in 0..count {
            let val = stacks[source].pop().unwrap();
            stacks[target].push(val)
        }
    }

    stacks.iter().filter_map(|s| s.last()).collect::<String>()
}

fn part2(input: &str) -> String {
    let mut lines = input.lines().peekable();
    let mut stacks = vec![];
    let stack_width = lines.peek().unwrap().len() / 4 + 1;
    for _ in 0..stack_width {
        stacks.push(vec![])
    }
    for line in lines.by_ref() {
        if line.starts_with(" 1 ") {
            break;
        }
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(idx, c)| {
                if c != ' ' {
                    stacks[idx].insert(0, c);
                }
            });
    }

    assert_eq!(lines.next().unwrap(), "");

    let instructions = lines.map(|l| {
        l.split(' ')
            .skip(1)
            .step_by(2)
            .map(|v| v.parse::<usize>().unwrap())
    });
    for mut ins in instructions {
        let count = ins.next().unwrap();
        let source = ins.next().unwrap() - 1;
        let target = ins.next().unwrap() - 1;
        let mut tmp = vec![];
        for _ in 0..count {
            tmp.push(stacks[source].pop().unwrap());
        }
        for _ in 0..count {
            stacks[target].push(tmp.pop().unwrap());
        }
    }

    stacks.iter().filter_map(|s| s.last()).collect::<String>()
}

#[cfg(test)]
mod test {

    use super::*;
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn day5_part1() {
        assert_eq!(part1(TEST_INPUT), "CMZ");
    }

    #[test]
    fn day5_part2() {
        assert_eq!(part2(TEST_INPUT), "MCD");
    }
}
