use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../../data/day7.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn run_cd(cwd: &mut String, arg: &str) {
    match arg {
        "/" => *cwd = "/".to_string(),
        ".." => {
            *cwd = cwd
                .strip_suffix('/')
                .unwrap()
                .rsplit_once('/')
                .unwrap()
                .0
                .to_string()
                + "/"
        }
        a => {
            cwd.push_str(a);
            cwd.push('/');
        }
    }
}
fn build_dirs(input: &str) -> BTreeMap<String, usize> {
    let mut cwd = String::new();
    let mut dirs: BTreeMap<_, usize> = BTreeMap::new();

    input.lines().for_each(|l| {
        if l.starts_with("$ cd") {
            run_cd(&mut cwd, &l[5..]);
            dirs.entry(cwd.clone()).or_default();
            return;
        }

        if l.starts_with("$ ls") {
            return;
        }

        if l.starts_with("dir ") {
            return;
        }

        let entry = dirs.get_mut(&cwd).expect("should have dir");

        let size = l.split_once(' ').unwrap().0.parse::<usize>().unwrap();
        *entry += size;
    });

    // update totals, depth-first
    let mut dirs2 = dirs.clone().into_keys().collect::<Vec<_>>();
    dirs2.sort();
    dirs2.reverse();
    for path in dirs2 {
        let size = *dirs.get(&path).unwrap();
        if let Some(pidx) = path.strip_suffix('/').unwrap().rfind('/') {
            let parent = &path[..pidx + 1];
            let value = dirs.get_mut(parent).expect("dir should exist now");
            *value += size;
        }
    }
    dirs
}

fn part1(input: &str) -> usize {
    let dirs = build_dirs(input);
    dirs.values().filter(|&&v| v <= 100000).sum()
}

fn part2(input: &str) -> usize {
    let dirs = build_dirs(input);
    let mut sorted_dirs = dirs.clone().into_iter().collect::<Vec<_>>();
    sorted_dirs.sort();
    let used = *dirs.get("/").unwrap();
    let max_allowed = 40_000_000;
    let overused = used
        .checked_sub(max_allowed)
        .expect("used less than max_allowed");

    let min_dir = dirs
        .into_iter()
        .filter(|kv| kv.1 >= overused)
        .min_by_key(|kv| kv.1)
        .unwrap();
    min_dir.1
}

#[cfg(test)]
mod tests {

    use super::*;
    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn day7_part1() {
        assert_eq!(part1(TEST_INPUT), 95437);
    }
    #[test]
    fn day7_part2() {
        assert_eq!(part2(TEST_INPUT), 24933642);
    }
}
