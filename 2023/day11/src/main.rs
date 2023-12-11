use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input, 2));
    println!("part 2: {}", part1(input, 1_000_000));
}

fn part1(input: &str, expansion: i64) -> i64 {
    let image = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _c)| (i as i64, j as i64))
        })
        .collect::<BTreeSet<_>>();

    let galaxies = expand(image, expansion);

    let mut dist = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            dist += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }
    dist
}

fn expand(image: BTreeSet<(i64, i64)>, expansion: i64) -> Vec<(i64, i64)> {
    let galaxy_rows = image
        .iter()
        .map(|coord| coord.0)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let galaxy_cols = image
        .iter()
        .map(|coord| coord.1)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let mut row_gaps = galaxy_rows
        .windows(2)
        .map(|v| (v[1] - v[0] - 1) * (expansion - 1))
        .collect::<Vec<_>>();
    let mut col_gaps = galaxy_cols
        .windows(2)
        .map(|v| (v[1] - v[0] - 1) * (expansion - 1))
        .collect::<Vec<_>>();

    // prefix sum
    for i in 1..row_gaps.len() {
        row_gaps[i] += row_gaps[i - 1]
    }
    for i in 1..col_gaps.len() {
        col_gaps[i] += col_gaps[i - 1]
    }

    image
        .into_iter()
        .map(|mut coord| {
            let row_idx = galaxy_rows.iter().position(|r| *r == coord.0).unwrap();
            if row_idx != 0 {
                coord.0 += row_gaps[row_idx - 1];
            }
            let col_idx = galaxy_cols.iter().position(|c| *c == coord.1).unwrap();
            if col_idx != 0 {
                coord.1 += col_gaps[col_idx - 1];
            }
            coord
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example1.txt");
        assert_eq!(part1(input, 2), 374);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../example1.txt");
        assert_eq!(part1(input, 10), 1030);
        assert_eq!(part1(input, 100), 8410);
    }
}
