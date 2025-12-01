use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
  type Output = i64;

  fn day_number(&self) -> u8 {
    1
  }

  fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
    let mut password = 0;

    input.into_iter()
      .fold(50, |dial, line| {
        let dial = turn_dial(dial, &line);
        if dial == 0 { password += 1; }
        dial
      });

    Ok(password)
  }

  fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
    

    Ok(0)
  }
}

fn turn_dial(dial: i64, rotation: &str) -> i64 {
    let side = rotation.chars().nth(0).unwrap();
    let amount = rotation[1..].parse::<i64>().unwrap();

    let dial = match side {
        'L' => dial - amount,
        'R' => dial + amount,
        _ => 0,
    } as i64;

    normalize_dial(dial)
}

fn normalize_dial(dial: i64) -> i64 {
    if dial < 0 {
        normalize_dial(99 + dial + 1)
    } else if dial > 99 {
        normalize_dial(dial - 99 - 1)
    } else {
        dial as i64
    }
}
