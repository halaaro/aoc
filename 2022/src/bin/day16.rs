#![allow(dead_code)]

use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
};

fn main() {
    let input = include_str!("../../data/day16.input");
    println!("part1: {}", part1(input));
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Valve<'a> {
    value: usize,
    name: &'a str,
    tunnels: Vec<usize>,
}

#[derive(Debug, Default, Clone)]
struct Graph<'a> {
    valves: Vec<Valve<'a>>,
}

impl<'a> Graph<'a> {
    fn index(&self, name: &str) -> Option<usize> {
        self.valves.iter().position(|v| v.name == name)
    }

    fn calc_score(&self, path: &[usize]) -> usize {
        let mut minutes = 30;
        let mut rate = 0usize;
        let mut score = 0;
        for win in path.windows(2) {
            let (from, to) = (win[0], win[1]);
            let path_time = match self.calc_dist(from, to) {
                Some(t) if t < minutes => t,
                _ => break,
            };
            score += (path_time + 1) * rate;
            minutes -= path_time + 1; // travel + open valve
            rate += self.valves[to].value;
        }
        score += rate * minutes;
        score
    }

    fn calc_dist(&self, from_idx: usize, to_idx: usize) -> Option<usize> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_front((from_idx, 0));

        while let Some((nb_idx, dist)) = queue.pop_back() {
            visited.insert(nb_idx);
            let node = &self.valves[nb_idx];
            if nb_idx == to_idx {
                return Some(dist);
            }
            for tun in node.tunnels.iter() {
                if !visited.contains(tun) {
                    queue.push_front((*tun, dist + 1));
                }
            }
        }
        None
    }
}

impl<'a> TryFrom<&'a str> for Graph<'a> {
    type Error = (&'static str, &'a str);
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut graph = Self::default();
        let mut edges = Vec::new();

        for line in input.lines() {
            let name = &line[6..8];
            let value = line
                .split_once('=')
                .ok_or(("failed to find '=' on line", line))?
                .1
                .split_once(';')
                .ok_or(("failed to find ';' on line", line))?
                .0
                .parse::<usize>()
                .or(Err(("failed to parse value on line", line)))?;

            let tunnels = line
                .split_once("valves ")
                .or_else(|| line.split_once("valve "))
                .ok_or(("failed to find valve or valves on line", line))?
                .1
                .split(", ");
            edges.push(tunnels);

            graph.valves.push(Valve {
                name,
                value,
                ..Valve::default()
            });
        }

        for (idx, tunnels) in edges.into_iter().enumerate() {
            for tunnel in tunnels {
                let pos = graph
                    .index(tunnel)
                    .ok_or(("failed to find node for tunnel", tunnel))?;
                graph.valves[idx].tunnels.push(pos);
            }
        }

        Ok(graph)
    }
}

/// find max pressure released, within timelimit
///
/// ## approach
/// 1. choose next unvisited node based on max flow rate
/// 2. instead of finding shortest path, find path with maximal score
/// 3. when visiting node, need to decide if we open valve or not
fn part1(input: &str) -> usize {
    let graph: Graph = input.try_into().expect("failed to parse graph");

    let mut score = 0;
    let mut path = vec![];
    for _ in 0..100 {
        // let mut nodes = BinaryHeap::from_iter(graph.valves.iter());
        let nodes: HashSet<Valve> =
            HashSet::from_iter(graph.valves.clone().into_iter().filter(|v| v.value > 0));
        let mut best_path = vec![0];
        let mut best_score = 0;

        for node in nodes {
            // println!("{:?}: {node:?}", graph.index(node.name));
            if node.value == 0 {
                continue;
            }
            let mut best_path_with_node = best_path.clone();
            let mut best_score_with_node = best_score;
            for idx in 0..best_path.len() {
                // println!(
                //     "attempt adding {} (idx {}) after position {}",
                //     node.name,
                //     graph.index(node.name).unwrap(),
                //     idx
                // );
                let mut path = best_path.clone();
                path.insert(idx + 1, graph.index(node.name).unwrap());
                let score = graph.calc_score(&path);
                // println!("experimental path: {:?} with score {}", &path, score);

                if score > best_score_with_node {
                    best_path_with_node = path;
                    best_score_with_node = score;
                    //     println!("updating best_path_with_node: {:?}", &best_path_with_node);
                }
            }

            if best_score_with_node > best_score {
                best_path = best_path_with_node;
                best_score = best_score_with_node;
                // println!("updating best_path: {:?}", &best_path);
            }
        }

        // println!(
        //     "best path: {:?}",
        //     best_path
        //         .into_iter()
        //         .map(|idx| graph.valves[idx].name)
        //         .collect::<Vec<_>>()
        // );

        score = max(best_score, score);
        if score == best_score {
            path = best_path
        }
    }
    // println!(
    //     "best path: {:?}",
    //     path.into_iter()
    //         .map(|idx| graph.valves[idx].name)
    //         .collect::<Vec<_>>()
    // );

    println!(
        "best path: {:?}",
        path.windows(2)
            .map(|i| (i[0], i[1]))
            .map(|(from, to)| format!(
                "{} --{}--> {}",
                graph.valves[from].name,
                graph.calc_dist(from, to).unwrap(),
                graph.valves[to].name
            ))
            .collect::<Vec<_>>()
    );
    let minutes_traveled : usize= path.windows(2)
            .map(|i| (i[0], i[1]))
            .map(|(from, to)|                graph.calc_dist(from, to).unwrap()
            )
            .sum();
    let minutes_turning_valves = path.len()-1;
    dbg!(minutes_traveled, minutes_turning_valves);

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
    #[test]
    fn day16_part1() {
        assert_eq!(part1(TEST_INPUT), 1651);
    }

    #[test]
    fn graph_calc_score() {
        let graph: Graph = TEST_INPUT.try_into().unwrap();
        assert_eq!(graph.calc_score(&[0, 1]), 364);
    }

    #[test]
    fn graph_calc_dist() {
        let graph: Graph = TEST_INPUT.try_into().unwrap();
        assert_eq!(graph.calc_dist(0, 1), Some(1));
        assert_eq!(graph.calc_dist(0, 2), Some(2));
    }
}
