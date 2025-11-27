use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
  type Output = String;

  fn day_number(&self) -> u8 {
    1
  }

  fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
    input.iter().for_each(|s| println!("{}", s));
    Ok("Day 1 part 1 placeholder answer".to_string())
  }

  fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
    input.iter().for_each(|s| println!("{}", s));
    Ok("Day 1 part 2 placeholder answer".to_string())
  }
}
