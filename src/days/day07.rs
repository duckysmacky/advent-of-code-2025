use std::collections::HashMap;

use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u64;

    fn day_number(&self) -> u8 {
        7
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let mut input_iter = input.iter();

        let top_row = input_iter.next().unwrap();
        let width = top_row.len();
        let start_col = top_row.find('S').unwrap();

        let mut row = vec!['.'; width];
        row[start_col] = '|';

        let mut count = 0;

        input_iter
            .for_each(|line| {
                line.chars()
                    .enumerate()
                    .for_each(|(col, c)| {
                        if c == '^' {
                            if row[col] == '|' {
                                row[col] = '^';
                                row[col - 1] = '|';
                                row[col + 1] = '|';
                                count += 1;
                            }
                        } else if c == '.' {
                            if row[col] == '^' {
                                row[col] = '.';
                            }
                        }
                    });
            });

        Ok(count)
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let start_col = input[0].find('S').unwrap();

        let mut cache = HashMap::new();
        let count = find(&mut cache, &input, 1, start_col);

        Ok(count)
    }
}

fn find(cache: &mut HashMap<(usize, usize), u64>, manifold: &Vec<String>, row: usize, col: usize) -> u64 {
    if row >= manifold.len() {
        return 1;
    }

    if col >= manifold[row].len() {
        return 0;
    }

    if let Some(&count) = cache.get(&(row, col)) {
        return count;
    }

    let count = match manifold[row].as_bytes()[col] {
        b'.' => find(cache, manifold, row + 1, col),
        b'^' => find(cache, manifold, row + 1, col - 1) + find(cache, manifold, row + 1, col + 1),
        _ => 0,
    };

    cache.insert((row, col), count);
    count
}