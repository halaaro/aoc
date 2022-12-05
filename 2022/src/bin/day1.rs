use std::cmp::Reverse;

fn main() {
    let input = include_str!("../../data/day1.input");
    let elves = input.split("\n\n").map(|c| {
        c.split('\n')
            .filter_map(|c0| c0.parse::<usize>().ok())
            .collect::<Vec<_>>()
    });


    let most_cals = elves.clone().enumerate().max_by_key(|e| e.1.iter().sum::<usize>()).unwrap();
    println!("elf with most cals: {:?}", (most_cals.0, most_cals.1.iter().sum::<usize>()));

    let mut sorted_elves : Vec<(usize,usize)> = elves.collect::<Vec<_>>().into_iter().enumerate().map(|e| (e.0, e.1.into_iter().sum())).collect();
    sorted_elves.sort_by_key(|e| Reverse(e.1));
    let top3 = sorted_elves.iter().take(3).collect::<Vec<_>>();
    println!("top 3 elves with most cals: {:?}", &top3);
    println!("top 3 total: {}", top3.iter().map(|e| e.1).sum::<usize>());
}
