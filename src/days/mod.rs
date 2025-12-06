use std::{fmt::{Debug, Display}, time::Instant};
use crate::util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

/// A generic trait which defines an solution for any one of the days.
/// This was added simply to reduce the boilerplate and code repetition
/// across the day solutions.
trait DaySolution : Default {
  /// The type of the output value, which the day's solution will produce
  type Output: Display + Debug;

  /// Public interface for running any day's solution. Will dynamically
  /// run the parts
  fn run(&self, part: Option<u8>, test: bool) -> Result<(), String> {
    match part {
      Some(part) => {
        self.run_part(part, test)
      }
      None => {
        self.run_part(1, test)?;
        self.run_part(2, test)
      }
    }
  }

  /// Middleware which runs the specifed part and print its result.
  /// Will also measure how much time it took to run
  fn run_part(&self, part: u8, test: bool) -> Result<(), String> {
    println!("[i] Running part #{}", part);

    let input = util::read_input(self.day_number(), part, test)?;
    let start = Instant::now();

    let result = match part {
      1 => self.part1(input),
      2 => self.part2(input),
      _ => Err("Invalid part provided".to_string())
    }?;

    let duration = start.elapsed();
    let total_ms = duration.as_secs_f64() * 1000f64;

    println!("[a] Part #{} answer: {}", part, result);
    println!("[i] Took {:.3}ms to run", total_ms);
    Ok(())
  }

  /// Returns day's number. Used in other functions
  fn day_number(&self) -> u8;
  /// Part 1 implementation
  fn part1(&self, input: Vec<String>) -> Result<Self::Output, String>;
  /// Part 2 implementation
  fn part2(&self, input: Vec<String>) -> Result<Self::Output, String>;
}

/// Runs the specified day. If the `part` parameter is `None`, will run all
/// solution parts for the given day, else will run the specified part
pub fn run_day(day: u8, part: Option<u8>, test: bool) -> Result<(), String> {
  println!("[i] Running day #{} solution", day);

  match day {
    1 => day01::Solution::default().run(part, test),
    2 => day02::Solution::default().run(part, test),
    3 => day03::Solution::default().run(part, test),
    4 => day04::Solution::default().run(part, test),
    5 => day05::Solution::default().run(part, test),
    6 => day06::Solution::default().run(part, test),
    _ => Err(format!("Day #{} solution not yet implemented", day))
  }
}
