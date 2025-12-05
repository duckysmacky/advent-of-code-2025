use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u64;

    fn day_number(&self) -> u8 {
        4
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let mut count = 0;

        for row in 0..input.len() {
            for col in 0..input[row].len() {
                if input[row].as_bytes()[col] == b'@' && count_rolls(&input, row, col) < 4 {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let mut input = input;
        let mut count = 0;

        let mut any_removed = true;
        while any_removed {
            any_removed = false;

            for row in 0..input.len() {
                for col in 0..input[row].len() {
                    if input[row].as_bytes()[col] == b'@' && count_rolls(&input, row, col) < 4 {
                        input[row].replace_range(col..=col, "x");
                        count += 1;
                        any_removed = true;
                    }
                }
            }
        }

        Ok(count)
    }
}

fn check_bounds(map: &Vec<String>, row: i32, col: i32) -> bool {
    row >= 0 && row < map.len() as i32 && col >= 0 && col < map[row as usize].len() as i32
}

fn count_rolls(map: &Vec<String>, row: usize, col: usize) -> u64 {
    let mut count = 0;

    let moves: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                ( 0, -1),          ( 0, 1),
                                ( 1, -1), ( 1, 0), ( 1, 1)];

    for move_i in 0..moves.len() {
        let row = row as i32 + moves[move_i].0 as i32;
        let col = col as i32 + moves[move_i].1 as i32;

        if check_bounds(map, row, col) && map[row as usize].as_bytes()[col as usize] == b'@' {
            count += 1;
        }
    }

    count
}