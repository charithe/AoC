#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

mod guard;
mod parse;

fn read_events_sorted() -> Result<Vec<parse::Entry>> {
    let input = File::open("input")?;
    let buffered = BufReader::new(input);

    let mut entries: Vec<parse::Entry> = buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| parse::entry(&line))
        .collect();

    entries.sort_by(|a, b| {
        a.year
            .cmp(&b.year)
            .then(a.month.cmp(&b.month))
            .then(a.day.cmp(&b.day))
            .then(a.hour.cmp(&b.hour))
            .then(a.minute.cmp(&b.minute))
    });

    Ok(entries)
}

fn main() -> Result<()> {
    let entries = read_events_sorted()?;
    let mut schedule = guard::Schedule::new();

    let mut current_guard: u32 = 0;
    let mut sleep_start: u8 = 0;

    for e in entries {
        match e.event {
            parse::Event::StartShift(id) => current_guard = id,
            parse::Event::FallAsleep => {
                sleep_start = e.minute;
            }
            parse::Event::WakeUp => {
                let sleep_end = e.minute;
                schedule.mark_asleep(current_guard, sleep_start, sleep_end);
            }
        }
    }

    if let Some((guard, most_asleep_minute)) = schedule.longest_sleeper_and_minute() {
        println!("Part 1 answer={}", guard * most_asleep_minute as u32);
    }

    if let Some((guard, most_asleep_minute)) = schedule.habitual_sleeper_and_minute() {
        println!("Part 2 answer={}", guard * most_asleep_minute as u32);
    }

    Ok(())
}
