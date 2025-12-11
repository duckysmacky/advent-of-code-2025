use std::collections::{HashSet, VecDeque};

use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u32;

    fn day_number(&self) -> u8 {
        10
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let total_presses = input.into_iter()
            .map(|line| parse_line(line))
            .map(|(diagram, buttons, _)| find_fewest_presses(diagram, buttons))
            .sum();

        Ok(total_presses)
    }

    fn part2(&self, _input: Vec<String>) -> Result<Self::Output, String> {
        Ok(0)
    }
}

fn parse_line(line: String) -> (Vec<bool>, Vec<Vec<u8>>, HashSet<u8>) {
    // 0 - light diagram, 1 - buttons, 2 - joltage
    let mut section = 0u8;

    let mut light_diagram = Vec::new();

    let mut buttons = Vec::new();
    let mut button = Vec::new();
    let mut light_n = 0u8;

    let mut joltage = HashSet::new();
    let mut joltage_n = 0u8;

    for c in line.chars() {
        match section {
            0 => {
                match c {
                    '.' => light_diagram.push(false),
                    '#' => light_diagram.push(true),
                    ' ' => section += 1,
                    _ => continue,
                }
            },
            1 => {
                match c {
                    '(' => {
                        button.clear();
                        light_n = 0;
                    },
                    ')' => {
                        button.push(light_n);
                        buttons.push(button.clone());
                        button.clear();
                        light_n = 0;
                    },
                    ',' => {
                        button.push(light_n);
                        light_n = 0;
                    },
                    '{' => section += 1,
                    _ => {
                        if let Some(digit) = c.to_digit(10) {
                            light_n = light_n * 10 + digit as u8;
                        } else {
                            continue;
                        }
                    }
                }
            },
            2 => {
                match c {
                    ',' => {
                        joltage.insert(joltage_n);
                        joltage_n = 0;
                    },
                    _ => {
                        if let Some(digit) = c.to_digit(10) {
                            joltage_n = joltage_n * 10 + digit as u8;
                        } else {
                            continue;
                        }
                    }
                }
            },
            _ => unreachable!(),
        }
    }

    (light_diagram, buttons, joltage)
}

fn find_fewest_presses(expected_diagram: Vec<bool>, buttons: Vec<Vec<u8>>) -> u32 {
    let mut fewest_presses = u32::MAX;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let empty_diagram = vec![false; expected_diagram.len()];
    queue.push_back((empty_diagram, 0u32));

    while let Some((current_diagram, presses)) = queue.pop_front() {
        if presses > fewest_presses {
            continue;
        }

        if current_diagram == expected_diagram {
            if presses < fewest_presses {
                fewest_presses = presses;
            }
            continue;
        }

        if !visited.insert(current_diagram.clone()) {
            continue;
        }

        for button in &buttons {
            let mut next_diagram = current_diagram.clone();

            for light_i in button {
                next_diagram[*light_i as usize] = !next_diagram[*light_i as usize];
            }

            queue.push_back((next_diagram, presses + 1));
        }
    }

    fewest_presses
}