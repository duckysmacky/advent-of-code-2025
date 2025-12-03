use crate::days::DaySolution;

#[derive(Default)]
pub struct Solution {}

impl DaySolution for Solution {
    type Output = u64;

    fn day_number(&self) -> u8 {
        2
    }

    fn part1(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let line = input.get(0).unwrap();

        let sum = line.split(",")
            .fold(0u64, |mut sum, range| {
                let mut segments = range.split("-");
                let start = segments.next().unwrap().parse::<u64>().unwrap();
                let end = segments.next().unwrap().parse::<u64>().unwrap();

                for n in start..=end {
                    let num = n.to_string();
                    let len = num.len();
                    
                    if len % 2 != 0 {
                        continue;
                    }

                    let left = &num[..len / 2];
                    let right = &num[len / 2..];

                    if left == right {
                        sum += n
                    }
                }

                sum
            });

        
        Ok(sum)
    }

    fn part2(&self, input: Vec<String>) -> Result<Self::Output, String> {
        let line = input.get(0).unwrap();

        let sum = line.split(",")
            .fold(0u64, |mut sum, range| {
                let mut segments = range.split("-");
                let start = segments.next().unwrap().parse::<u64>().unwrap();
                let end = segments.next().unwrap().parse::<u64>().unwrap();

                for n in start..=end {
                    let num = n.to_string();

                    if check_num(num) {
                        sum += n
                    }
                }

                sum
            });

        Ok(sum)
    }
}

fn check_num(num: String) -> bool {
    let len = num.len();
    let mut cut_size = len / 2;

    'cut: while cut_size > 0 {
        let cut = &num[..cut_size];

        if len % cut_size != 0 {
            cut_size -= 1;
            continue;
        }

        for i in 1..len / cut_size {
            let from = cut_size * i;
            let to = from + cut_size;
            let curr_cut = &num[from..to];

            if curr_cut != cut {
                cut_size -= 1;
                continue 'cut;
            }
        }

        return true;
    }

    false
}
