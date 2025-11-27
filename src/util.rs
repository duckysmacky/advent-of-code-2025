use std::{fs::File, io::{BufRead, BufReader}};

/// Reads the input file for specified day and part. Will return a
/// String error if something goes wrong
pub fn read_input(day: u8, part: u8) -> Result<Vec<String>, String> {
    let path = format!("input/day{}/{}.txt", day, part);
    let file = File::open(&path).map_err(|err| {
      format!("Unable to open '{}': {}", path, err)
    })?;

    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for (i, line) in reader.lines().enumerate() {
      let line = line.map_err(|err| {
        format!("Invalid bytes at line #{} of '{}': {}", i, path, err)
      })?;
      lines.push(line);
    }

    Ok(lines)
}
