use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u64;

    fn day_number(&self) -> u8 {
        8
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let parsed = input.into_iter()
            .map(|line| line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .take(3)
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();

        println!("{:?}", parsed);

        Ok(0)
    }

    fn part2(&self, _input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}

fn dist_slice(p: &[u32], q: &[u32]) -> f32 {
    ((0..3).into_iter()
        .map(|i| (p[i] - q[i]).pow(2))
        .sum::<u32>() as f32)
        .sqrt()
}

fn dist_tuple(p: (u32, u32, u32), q: (u32, u32, u32)) -> f32 {
    (((p.0 - q.0).pow(2) + (p.1 - q.1).pow(2) + (p.2 - q.2).pow(2)) as f32).sqrt()
}
