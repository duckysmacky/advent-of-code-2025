use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::{Ordering, min, max};

use crate::days::DaySolution;

/// Wrapper struct for f64 that implements Ord and Eq traits so I can use it in a BinaryHeap
#[derive(Debug, PartialEq, PartialOrd)]
struct Orderedf64(f64);

impl Eq for Orderedf64 {}

impl Ord for Orderedf64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u64;

    fn day_number(&self) -> u8 {
        8
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let mut points: Vec<[u64; 3]> = Vec::new();
        let mut calculated_indexes: HashSet<(usize, usize)> = HashSet::new();
        let mut connections = BinaryHeap::new();

        input.into_iter()
            .enumerate()
            .for_each(|(i, line)| {
                let mut point = [0u64; 3];

                line.split(',')
                    .map(|x| x.parse::<u64>().unwrap())
                    .enumerate()
                    .for_each(|(i, x)| point[i] = x);

                points.iter()
                    .enumerate()
                    .for_each(|(other_i, other_point)| {
                        let dist = distance(&point, other_point);
                        let indexes = (min(i, other_i), max(i, other_i));

                        if dist != 0.0 && !calculated_indexes.contains(&indexes) {
                            connections.push((Orderedf64(dist), i, other_i));
                            calculated_indexes.insert(indexes);
                        }
                    });

                points.push(point)
            });

        let mut connection_graph: HashMap<usize, Vec<usize>> = HashMap::new();

        connections.into_sorted_vec().into_iter()
            .take(1000)
            .for_each(|(_, origin_i, other_i)| {
                connection_graph.entry(origin_i)
                    .or_default()
                    .push(other_i);

                connection_graph.entry(other_i)
                    .or_default()
                    .push(origin_i);
            });

        let mut visited = HashSet::new();
        let mut sizes = BinaryHeap::new();

        for &start_node in connection_graph.keys() {
            if !visited.contains(&start_node) {
                let count = dfs_count(&connection_graph, &mut visited, start_node);
                sizes.push(count);
            }
        }

        let answer = sizes.into_sorted_vec().iter()
            .rev()
            .take(3)
            .product::<u64>();

        Ok(answer)
    }

    fn part2(&self, _input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}

fn distance(p: &[u64], q: &[u64]) -> f64 {
    ((0..3).into_iter()
        .map(|i| (p[i] as i64 - q[i] as i64).pow(2))
        .sum::<i64>() as f64)
        .sqrt()
}

fn dfs_count(graph: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<usize>, node: usize) -> u64 {
    if !visited.insert(node) {
        return 0;
    }

    let mut count = 0;

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                count += dfs_count(graph, visited, neighbor);
            }
        }
    }

    count + 1
}