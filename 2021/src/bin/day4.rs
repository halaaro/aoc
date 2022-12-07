fn main() {
    let input = include_str!("../../data/day4.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let nums = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap());

    let mut boards = input
        .lines()
        .skip(1)
        .collect::<Vec<_>>()
        .chunks(6)
        .map(|b| {
            b[1..]
                .iter()
                .flat_map(|line| {
                    line.split(' ')
                        .filter_map(|v| v.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for num in nums {
        for board in boards.iter_mut() {
            update_board(board, num);
            if board_winner(board) {
                return board_score(board) * num;
            }
        }
    }
    unreachable!()
}

fn board_score(board: &[usize]) -> usize {
    board.iter().filter(|&&v| v != 100).sum()
}

fn board_winner(board: &[usize]) -> bool {
    for col in 0..5 {
        if board.iter().skip(col).step_by(5).all(|&v| v == 100) {
            return true;
        }
    }
    board.chunks(5).any(|row| row.iter().all(|&v| v == 100))
}

fn update_board(board: &mut [usize], num: usize) {
    board.iter_mut().for_each(|v| {
        if v == &num {
            *v = 100
        }
    });
}

fn part2(input: &str) -> usize {
    let nums = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap());

    let mut boards = input
        .lines()
        .skip(1)
        .collect::<Vec<_>>()
        .chunks(6)
        .map(|b| {
            b[1..]
                .iter()
                .flat_map(|line| {
                    line.split(' ')
                        .filter_map(|v| v.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for num in nums {
        let mut boards_to_remove: Vec<usize> = vec![];
        let boards_count = boards.len();

        for (idx, board) in boards.iter_mut().enumerate() {
            update_board(board, num);
            if board_winner(board) {
                if boards_count == 1 {
                    return board_score(board) * num;
                } else {
                    boards_to_remove.push(idx);
                }
            }
        }
        for idx in boards_to_remove.into_iter().rev() {
            boards.remove(idx);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
    #[test]
    fn day4_part1() {
        assert_eq!(part1(TEST_INPUT), 4512);
    }

    #[test]
    fn day4_part2() {
        assert_eq!(part2(TEST_INPUT), 1924);
    }

    #[test]
    fn board_winner_test_row() {
        assert!(board_winner(&[
            100, 100, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]));
    }
    #[test]
    fn board_winner_test_col() {
        assert!(board_winner(&[
            100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0,
        ]));
    }
    #[test]
    fn board_score_test() {
        assert_eq!(
            board_score(&[
                100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0,
                0,
            ]),
            0
        );
        assert_eq!(
            board_score(&[
                100, 1, 0, 0, 0, 100, 1, 0, 0, 0, 100, 1, 0, 0, 0, 100, 1, 0, 0, 0, 100, 1, 0, 0,
                0,
            ]),
            5
        );
    }
    #[test]
    fn board_update_test() {
        let board = &mut [
            100, 99, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0,
        ];
        update_board(board, 99);
        dbg!(&board);
        assert_eq!(
            board,
            &[
                100, 100, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0, 0, 100, 0, 0, 0,
                0,
            ]
        );
    }
}
