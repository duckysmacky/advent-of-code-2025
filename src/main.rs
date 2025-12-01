use std::process;

mod util;
mod days;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    eprintln!("Error: invalid amount of arguments provided!");
    print_usage();
    process::exit(1);
  }

  let day = args[1].parse::<u8>().unwrap_or(0);
  let part = if args.len() > 2 {
    Some(args[2].parse::<u8>().unwrap_or(0))
  } else {
    None
  };

  if day < 1 || day > 12 {
    eprintln!("Error: Invalid day specified");
    process::exit(1);
  }

  if let Some(part) = part {
    if ![1, 2].contains(&part) {
      eprintln!("Error: Invalid part specified");
      process::exit(1);
    }
  }

  if let Err(err) = days::run_day(day, part) {
    eprintln!("Error running day #{}: {}", day, err);
    process::exit(1);
  }
}

fn print_usage() {
  println!("Usage: aoc2025 <day> [part]");
}
