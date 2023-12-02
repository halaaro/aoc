use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn game(i: &str) -> IResult<&str, u32> {
    use nom::character::complete::u32;
    let (i, (_, num, _)) = tuple((tag("Game "), u32, tag(": ")))(i)?;
    Ok((i, num))
}

fn num_color_pair(i: &str) -> IResult<&str, (u32, &str)> {
    use nom::character::complete::u32;
    separated_pair(u32, tag(" "), alpha1)(i)
}

#[derive(Debug, Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse_games(input: &str) -> std::vec::Vec<(usize, std::vec::Vec<Draw>)> {
    input
        .lines()
        .enumerate()
        .map(|(_i, line)| {
            let (input, game_num) = game(line).unwrap();
            let game_num = game_num as usize;
            let (input, draws) =
                separated_list0(tag("; "), separated_list0(tag(", "), num_color_pair))(input)
                    .unwrap();
            assert!(input.is_empty());

            let draws = draws
                .into_iter()
                .map(|d| {
                    let mut draw = Draw::default();
                    for (num, color) in d {
                        match color {
                            "red" => draw.red = num,
                            "blue" => draw.blue = num,
                            "green" => draw.green = num,
                            _ => unreachable!(),
                        }
                    }
                    draw
                })
                .collect::<Vec<_>>();
            (game_num, draws)
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    let max_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = parse_games(input).into_iter().filter(|(_, draws)| {
        draws
            .iter()
            .all(|d| d.red <= max_draw.red && d.green <= max_draw.green && d.blue <= max_draw.blue)
    });
    // println!("{:?}", games.clone().collect::<Vec<_>>());
    games.map(|g| g.0).sum()
}

fn part2(input: &str) -> u32 {
    let games = parse_games(input)
        .into_iter()
        .map(|(_, draws)| {
            draws.into_iter().fold(Draw::default(), |mut dmin, d| {
                dmin.red = dmin.red.max(d.red);
                dmin.green = dmin.green.max(d.green);
                dmin.blue = dmin.blue.max(d.blue);
                dmin
            })
        })
        .map(Draw::power)
        .collect::<Vec<_>>();

    // println!("{:?}", &games);
    games.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example1.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE_INPUT), 8);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE_INPUT), 2286);
    }
}
