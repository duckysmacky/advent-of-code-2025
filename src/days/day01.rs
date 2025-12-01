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
        let dial = normalize_dial(dial);
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
  let mut chars = rotation.chars();
  let side = chars.next().unwrap();
  let amount = chars.as_str().parse::<i64>().unwrap();

  let dial = match side {
    'L' => dial - amount,
    'R' => dial + amount,
    _ => 0,
  } as i64;

  dial
}

fn normalize_dial(dial: i64) -> i64 {
  if dial > 99 {
    dial % 100
  } else if dial < 0 {
    dial.rem_euclid(100)
  } else {
    dial
  }
}

// 99 + 5 = 104 -> 4
// 3 - 7 = -4 -> 96
//
//
//
//
//