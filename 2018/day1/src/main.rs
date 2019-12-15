use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let changes = read_change_list()?;
    println!("=====Imperative=====");
    imperative(&changes);
    println!("=====Functional======");
    functional(&changes);

    Ok(())
}

fn read_change_list() -> Result<Vec<i32>> {
    let input = File::open("input")?;
    let buffered = BufReader::new(input);

    let changes: Vec<_> = buffered
        .lines()
        .filter_map(|line| line.ok())
        .map(|x| x.parse::<i32>())
        .filter_map(|x| x.ok())
        .collect();

    Ok(changes)
}

fn imperative(changes: &Vec<i32>) {
    let mut freq: i32 = 0;
    let mut freq_hist: HashSet<i32> = HashSet::new();

    freq_hist.insert(freq);

    for change in changes {
        freq += change;
        freq_hist.insert(freq);
    }

    println!("Final frequency: {}", freq);

    'outer: loop {
        for change in changes {
            freq += change;
            if !freq_hist.insert(freq) {
                println!("Repeated frequency: {}", freq);
                break 'outer;
            }
        }
    }
}

fn functional(changes: &Vec<i32>) {
    let mut freq_hist: HashSet<i32> = HashSet::new();

    let final_freq = changes.iter().fold(0, |acc, &x| {
        let freq = acc + x;
        freq_hist.insert(freq);
        freq
    });

    println!("Final frequency: {}", final_freq);

    let repeated_freq = changes
        .iter()
        .cycle()
        .scan(final_freq, |state, &x| {
            *state = *state + x;
            Some(*state)
        })
        .skip_while(|x| freq_hist.insert(*x))
        .take(1)
        .next()
        .unwrap();

    println!("Repeated frequency: {}", repeated_freq)
}
