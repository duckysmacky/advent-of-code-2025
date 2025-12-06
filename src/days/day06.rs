use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u64;

    fn day_number(&self) -> u8 {
        6
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let lines = input.len();
        let mut totals = vec![0; input[0].len()];

        let operators = input[lines - 1]
            .split_whitespace()
            .enumerate()
            .map(|(i, op)| match op {
                "*" => {
                    totals[i] = 1;
                    |total, x| total * x
                }
                "+" => {
                    totals[i] = 0;
                    |total, x| total + x
                }
                _ => panic!("Invalid operator"),
            })
            .collect::<Vec<_>>();

        input.into_iter()
            .take(lines - 1)
            .for_each(|line| line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .enumerate()
                .for_each(|(i, x)| {
                    let total = totals[i];
                    totals[i] = operators[i](total, x);
                })
            );

        let result = totals.into_iter()
            .sum();

        Ok(result)
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}
