use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = usize;

    fn day_number(&self) -> u8 {
        5
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let mut input_iter = input.into_iter();

        let fresh_ranges = input_iter.by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split('-');
                let start = parts.next().unwrap().parse::<u64>().unwrap();
                let end = parts.next().unwrap().parse::<u64>().unwrap();
                (start, end)
            })
            .collect::<Vec<_>>();

        let fresh_ids = input_iter
            .map(|line| line.parse::<u64>().unwrap())
            .filter(|id| {
                fresh_ranges.iter()
                    .any(|&(start, end)| start <= *id && *id <= end)
            })
            .count();

        Ok(fresh_ids)
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}