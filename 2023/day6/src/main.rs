fn main() {
    let input = &[(53, 250), (91, 1330), (67, 1081), (68, 1025)];
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part1(&[(53_91_67_68, 250_1330_1081_1025)]));
}

// brute force
// fn part1(input: &[(u64, u64)]) -> usize {
//     use rayon::prelude::*;
//     input
//         .iter()
//         .map(|&(tot_time, record)| {
//             (0..tot_time)
//                 .into_par_iter()
//                 .filter(|button_time| (tot_time - button_time) * button_time > record)
//                 .count()
//         })
//         .product()
// }

// binary search
// fn part1(input: &[(usize, usize)]) -> usize {
//     input
//         .iter()
//         .map(|&(tot_time, record)| {
//             let mut a = 0;
//             let mut b = tot_time / 2; // peak of parabola
//             while a < b {
//                 let c = a + (b - a) / 2;
//                 let score = (tot_time - c) * c;
//                 if score == record {
//                     a = c;
//                     break;
//                 }
//                 if score > record {
//                     b = c - 1;
//                 } else if a < c {
//                     a = c;
//                 } else {
//                     b += 1
//                 }
//             }
//             (tot_time + 1) - (a + 1) * 2
//         })
//         .product()
// }

// fixed-precision calculation
fn part1(input: &[(i64, i64)]) -> usize {
    input
        .iter()
        .map(|&(tot_time, record)| {
            let tt = tot_time * 100;
            let rd = record * 10_000;
            let mut bt = tt / 4; // initial guess
            for _ in 0..30 {
                // accelerated newton-raphson
                let err = (tt - bt) * bt - rd;
                let m = tt - 2 * bt;
                bt -= (err / m * 3) / 2;
            }
            let button_time = (bt + 1) / 100;
            (tot_time + 1) - (button_time + 1) * 2
        })
        .product::<i64>() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = &[(7, 9), (15, 40), (30, 200)];
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part1(&[(71530, 940200)]), 71503);
    }
}
