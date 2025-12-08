use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::{Ordering, min, max};

use crate::days::DaySolution;

/// Wrapper struct for f32 that implements Ord and Eq traits so I can use it in a BinaryHeap
#[derive(Debug, PartialEq, PartialOrd)]
struct OrderedF32(f32);

impl Eq for OrderedF32 {}

impl Ord for OrderedF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = usize;

    fn day_number(&self) -> u8 {
        8
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let points = input.into_iter()
            .map(|line| {
                let mut coordinates = [0u32; 3];

                line.split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .enumerate()
                    .for_each(|(i, x)| coordinates[i] = x);

                coordinates
            })
            .collect::<Vec<_>>();

        let mut calculated_indexes: HashSet<(usize, usize)> = HashSet::new();
        let mut connections = BinaryHeap::new();

        for origin_i in 0..points.len() {
            println!("{} - {:?}", origin_i, points[origin_i]);
            let origin_point = &points[origin_i];

            for other_i in 0..points.len() {
                let other_point = &points[other_i];
                let dist = distance(origin_point, other_point);
                let indexes = (min(origin_i, other_i), max(origin_i, other_i));

                if dist != 0.0 && !calculated_indexes.contains(&indexes) {
                    connections.push((OrderedF32(-dist), origin_i, other_i));
                    calculated_indexes.insert(indexes);
                }
            }
        }

        let mut connection_graph: HashMap<usize, HashSet<usize>> = HashMap::new();

        let best_connections = connections.into_iter()
            .take(10)
            .collect::<Vec<_>>();

        for (_, origin_i, other_i) in best_connections {
            println!("Connecting {} and {}", origin_i, other_i);

            connection_graph.entry(origin_i)
                .or_default()
                .insert(other_i);

            connection_graph.entry(other_i)
                .or_default()
                .insert(origin_i);
        }

        eprintln!("Connections:");
        for (i, points) in &connection_graph {
            println!("{}: {:?}", i, points);
        }

        let mut visited = HashSet::new();
        let mut sizes = BinaryHeap::new();

        for &start_node in connection_graph.keys() {
            if !visited.contains(&start_node) {
                println!("Start node: {}", start_node);
                let count = dfs_count(&connection_graph, &mut visited, start_node);
                sizes.push(count);
            }
        }

        println!("Sizes: {:?}", sizes);

        let answer = sizes.into_iter()
            .take(3)
            .fold(1, |total, x| total * x);

        Ok(answer as usize)
    }

    fn part2(&self, _input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}

fn distance(p: &[u32], q: &[u32]) -> f32 {
    ((0..3).into_iter()
        .map(|i| (p[i] as i32 - q[i] as i32).pow(2))
        .sum::<i32>() as f32)
        .sqrt()
}

fn dfs_count(graph: &HashMap<usize, HashSet<usize>>, visited: &mut HashSet<usize>, node: usize) -> u32 {
    if !visited.insert(node) {
        return 0;
    }

    println!("Visiting node: {}", node);

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