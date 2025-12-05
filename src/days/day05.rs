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

        let ranges = input_iter.by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split('-');
                let start = parts.next().unwrap().parse::<u64>().unwrap();
                let end = parts.next().unwrap().parse::<u64>().unwrap();
                (start, end)
            })
            .collect::<Vec<_>>();

        let ranges = merge_ranges(ranges);

        let fresh_ids = input_iter
            .map(|line| line.parse::<u64>().unwrap())
            .filter(|id| is_in_ranges(&ranges, *id))
            .count();

        Ok(fresh_ids)
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let ranges = input.into_iter()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split('-');
                let start = parts.next().unwrap().parse::<u64>().unwrap();
                let end = parts.next().unwrap().parse::<u64>().unwrap();
                (start, end)
            })
            .collect::<Vec<_>>();

        let ranges_count = merge_ranges(ranges).into_iter()
            .map(|(start, end)| end - start + 1)
            .sum::<u64>();

        Ok(ranges_count as usize)
    }
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current = ranges[0];

    ranges.into_iter()
        .skip(1)
        .for_each(|range| {
            if range.0 <= current.1 + 1 {
                current.1 = current.1.max(range.1);
            } else {
                merged.push(current);
                current = range;
            }
        });

    merged.push(current);
    merged
}

fn is_in_ranges(ranges: &[(u64, u64)], id: u64) -> bool {
    let mut left = 0;
    let mut right = ranges.len();

    while left < right {
        let mid = left + (right - left) / 2;
        let (start, end) = ranges[mid];

        if id < start {
            right = mid;
        } else if id > end {
            left = mid + 1;
        } else {
            return true;
        }
    }
    false
}