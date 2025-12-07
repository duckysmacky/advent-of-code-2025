use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u32;

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

    fn part2(&self, _input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}