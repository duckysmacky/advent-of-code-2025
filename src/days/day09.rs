use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u64;

    fn day_number(&self) -> u8 {
        9
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let mut tiles: Vec<[u64; 2]> = Vec::new();
        let mut best_area = 0;

        for line in input {
            let mut tile = [0u64; 2];

            line.split(',')
                .map(|x| x.parse::<u64>().unwrap())
                .enumerate()
                .for_each(|(i, x)| tile[i] = x);

            for other_tile in &tiles {
                let area = (0..2).into_iter()
                    .map(|i| tile[i].abs_diff(other_tile[i]) + 1)
                    .fold(1, |area, x| area * x);

                best_area = best_area.max(area);
            }

            tiles.push(tile);
        }

        Ok(best_area)
    }

    fn part2(&self, _input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}
