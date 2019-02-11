use std::fs::File;
use std::io::{Result, BufRead, BufReader};
use std::collections::HashSet;

fn main() -> Result<()> {
    let file_path = "input";
    let input = File::open(file_path)?;
    let buffered = BufReader::new(input);

    let mut freq: i32 = 0;
    let mut freq_hist: HashSet<i32> = HashSet::new();
    let mut change_list: Vec<i32> = Vec::new();

    freq_hist.insert(freq);

    for line in buffered.lines() {
        let change = line?.parse::<i32>().unwrap();
        change_list.push(change);
        freq += change;
        freq_hist.insert(freq);
    }

    println!("Final frequency: {}", freq);

    'outer: loop {
        for change in &change_list  {
            freq += change;
            if !freq_hist.insert(freq) {
    println!("Repeated frequency: {}", freq);
                break 'outer;
            }
        }
    }

    Ok(())
}

