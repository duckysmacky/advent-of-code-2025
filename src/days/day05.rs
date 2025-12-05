use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = usize;

    fn day_number(&self) -> u8 {
        5
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let blank_line_i = input.iter()
            .position(|line| line.is_empty())
            .unwrap();

        let (input_ranges, input_ids) = input.split_at(blank_line_i);
        let input_ids = &input_ids[1..];

        let fresh_ranges = input_ranges.into_iter()
            .map(|line| {
                let mut parts = line.split('-');
                let start = parts.next().unwrap().parse::<u64>().unwrap();
                let end = parts.next().unwrap().parse::<u64>().unwrap();
                (start, end)
            })
            .collect::<Vec<_>>();

        let fresh_ids = input_ids.into_iter()
            .map(|line| line.parse::<u64>().unwrap())
            .filter(|id| {
                fresh_ranges.iter()
                    .any(|&(start, end)| start <= *id && *id <= end)
            })
            .collect::<Vec<_>>();

        // println!("{:?}", fresh_ranges);
        // println!("{:?}", fresh_ids);

        Ok(fresh_ids.len())
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}