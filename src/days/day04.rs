use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u64;

    fn day_number(&self) -> u8 {
        4
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let map = input.into_iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut count = 0;

        for row in 0..map.len() {
            for col in 0..map[row].len() {
                if map[row][col] == '@' && count_rolls(&map, row, col) < 4 {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}

fn check_bounds(map: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    row >= 0 && row < map.len() as i32 && col >= 0 && col < map[row as usize].len() as i32
}

fn count_rolls(map: &Vec<Vec<char>>, row: usize, col: usize) -> u64 {
    let mut count = 0;

    let moves: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                ( 0, -1),          ( 0, 1),
                                ( 1, -1), ( 1, 0), ( 1, 1)];

    for move_i in 0..moves.len() {
        let row = row as i32 + moves[move_i].0 as i32;
        let col = col as i32 + moves[move_i].1 as i32;

        if check_bounds(map, row, col) && map[row as usize][col as usize] == '@' {
            count += 1;
        }
    }

    count
}