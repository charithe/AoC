use aoc;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> aoc::Result<()> {
    let masses = read_masses()?;
    let part1_total = masses.iter().fold(0, |acc, x| acc + (x / 3 - 2));
    println!("Part1: {}", part1_total);

    let part2_total = masses.iter().fold(0, |acc, x| acc + calc_fuel(*x));
    println!("Part2: {}", part2_total);

    Ok(())
}

fn read_masses() -> aoc::Result<Vec<i32>> {
    let input = File::open("input")?;
    let buffered = BufReader::new(input);

    buffered.lines().map(|line| aoc::parse_i32(line)).collect()
}

fn calc_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + calc_fuel(fuel)
    }
}
