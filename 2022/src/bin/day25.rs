use std::fmt::Display;
use std::str::FromStr;

fn main() {
    let input = include_str!("../../data/day25.input");
    println!("part1: {}", part1(input));
}

struct Snafu(usize);
impl Snafu {
    fn format(&self) -> String {
        let mut buf = vec![];
        let mut val = self.0;
        if val == 0 {
            return "0".to_string();
        }
        while val > 0 {
            let remainder = val % 5;
            let (chr, adj) = match remainder {
                0 => ("0", 0),
                1 => ("1", 0),
                2 => ("2", 0),
                3 => ("=", 1),
                4 => ("-", 1),
                _ => unreachable!(),
            };
            buf.push(chr);
            val /= 5;
            val += adj;
        }

        buf.reverse();
        buf.concat()
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format())
    }
}

impl FromStr for Snafu {
    type Err = (&'static str, char);
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "0" {
            return Ok(Snafu(0));
        }
        let mut val: i64 = 0;
        for c in s.chars() {
            val *= 5;
            val += match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '=' => -2,
                '-' => -1,
                c => return Err(("invalid input", c)),
            }
        }
        Ok(Snafu(val as usize))
    }
}

fn part1(input: &str) -> String {
    let snafu_sum = input
        .lines()
        .map(|line| line.parse::<Snafu>().unwrap())
        .map(|s| s.0)
        .sum();

    Snafu(snafu_sum).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";
    #[test]
    fn snafu_format() {
        assert_eq!(&Snafu(0).format(), "0");
        assert_eq!(&Snafu(1).format(), "1");
        assert_eq!(&Snafu(2).format(), "2");
        assert_eq!(&Snafu(3).format(), "1=");
        assert_eq!(&Snafu(4).format(), "1-");
        assert_eq!(&Snafu(5).format(), "10");
        assert_eq!(&Snafu(10).format(), "20");
        assert_eq!(&Snafu(15).format(), "1=0");
    }
    #[test]
    fn day25_part1() {
        assert_eq!(&part1(TEST_INPUT), "2=-1=0");
    }
}
