use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u32;

    fn day_number(&self) -> u8 {
        3
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let total = input.into_iter()
            .fold(0u32, |total, line| {
                let mut pair = (0, 0);
                let mut best_num = 0;

                line.chars().into_iter()
                    .map(|c| c.to_digit(10).unwrap())
                    .enumerate()
                    .for_each(|(i, val)| {
                        if val > pair.0 && i != line.len() - 1 {
                            best_num = best_num.max(num_from_pair(pair));
                            pair.0 = val;
                            pair.1 = 0;
                        } else if val > pair.1 {
                            pair.1 = val;
                            best_num = best_num.max(num_from_pair(pair));
                        }
                    });

                total + best_num
            });

        Ok(total)
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}

fn num_from_pair(pair: (u32, u32)) -> u32 {
    pair.0 * 10 + pair.1
}
