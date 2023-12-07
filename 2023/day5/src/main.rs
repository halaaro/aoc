fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = lines.filter_map(|line| {
        if let Some(c) = line.chars().next() {
            if c.is_ascii_alphabetic() {
                return None;
            }
        }
        Some(
            line.split_ascii_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        )
    });

    let mut round = seeds.clone();

    for map in maps {
        if map.is_empty() {
            seeds = round.clone();
            continue;
        }
        let [dst, src, cnt] = map[..3] else { todo!("handle bad input") };
        let seed_len = seeds.len();
        for i in 0..seed_len {
            let mut seed = seeds[i];
            if seed >= src && seed < src + cnt {
                seed += dst;
                seed -= src;
                round[i] = seed;
            }
        }
    }
    round.into_iter().min().unwrap()
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut seeds = seeds
        .chunks_exact(2)
        .map(|cnk| (cnk[0], cnk[0] + cnk[1] - 1))
        .collect::<Vec<_>>();

    let maps = lines.filter_map(|line| {
        if let Some(c) = line.chars().next() {
            if c.is_ascii_alphabetic() {
                return None;
            }
        }
        Some(
            line.split_ascii_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        )
    });

    let mut round = seeds.clone();

    for map in maps {
        if map.is_empty() {
            seeds = round.clone();
            continue;
        }
        let [dst, src, cnt] = map[..3] else { todo!() };
        let src_rng = (src, src + cnt - 1);
        let seed_len = seeds.len();
        for i in 0..seed_len {
            let seed = seeds[i];
            if seed.0 <= src_rng.1 && seed.1 >= src_rng.0 {
                // ranges split
                if seed.0 < src_rng.0 {
                    seeds.push((seed.0,src_rng.0-1));
                    round.push(seeds[seeds.len()-1]);
                }
                if seed.1 > src_rng.1 {
                    seeds.push((src_rng.1 +1, seed.1));
                    round.push(seeds[seeds.len()-1]);
                }
                // resize current range if needed
                seeds[i] = (seed.0.max(src_rng.0), seed.1.min(src_rng.1));
                round[i] = ((seeds[i].0 + dst) - src, (seeds[i].1 + dst) - src);
            }
        }
    }
    round.into_iter().map(|r| r.0).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example1.txt");
        assert_eq!(part1(input), 35);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("../example1.txt");
        assert_eq!(part2(input), 46);
    }
}
