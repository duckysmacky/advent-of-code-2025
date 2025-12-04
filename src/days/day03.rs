use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u64;

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

        Ok(total as u64)
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let total = input.into_iter()
            .fold(0u64, |total, line| {
                let nums = line.chars().into_iter()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>();

                let mut num_digits = [0u8; 12];

                let mut digit_i = 0;
                let mut start_i = 0;
                for num_i in (nums.len() - 12)..nums.len() {
                    let mut max_num = nums[num_i];
                    let mut found_max = false;

                    for max_i in start_i..num_i {
                        if nums[max_i] > max_num || (nums[max_i] == max_num && !found_max) {
                            max_num = nums[max_i];
                            start_i = max_i + 1;
                            found_max = true;
                        }
                    }

                    if start_i <= num_i && !found_max {
                        start_i = num_i + 1;
                    }

                    num_digits[digit_i] = max_num;
                    digit_i += 1;
                }

                let num = num_digits.into_iter()
                    .enumerate()
                    .fold(0, |num, (i, val)| {
                        num + 10u64.pow(12 - i as u32 - 1) * val as u64
                    });

                total + num
            });

        Ok(total)
    }
}

fn num_from_pair(pair: (u32, u32)) -> u32 {
    pair.0 * 10 + pair.1
}
