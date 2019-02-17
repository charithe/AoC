use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

struct IdLetters(u32, u32);

fn main() -> Result<()> {
    let id_list = read_id_list()?;
    part1(&id_list);
    part2(&id_list);
    Ok(())
}

fn read_id_list() -> Result<Vec<String>> {
    let input = File::open("input")?;
    let buffered = BufReader::new(input);

    let id_list = buffered
        .lines()
        .filter_map(|line| line.ok())
        .sorted()
        .collect();
    Ok(id_list)
}

fn part1(id_list: &Vec<String>) {
    let letter_sums = id_list
        .iter()
        .map(count_id_letters)
        .fold(IdLetters(0, 0), |acc, v| {
            IdLetters(acc.0 + v.0, acc.1 + v.1)
        });

    println!("Checksum: {}", letter_sums.0 * letter_sums.1)
}

fn count_id_letters(id: &String) -> IdLetters {
    id.chars()
        .fold(HashMap::new(), |mut acc, c| {
            let count = acc.entry(c).or_insert(0);
            *count += 1;
            acc
        })
        .values()
        .fold(IdLetters(0, 0), |acc, count| match count {
            2 => IdLetters(1, acc.1),
            3 => IdLetters(acc.0, 1),
            _ => acc,
        })
}

fn part2(id_list: &Vec<String>) {
    let ids = id_list.as_slice();
    let mut n = 0;
    'outer: loop {
        for i in (n + 1)..ids.len() {
            match diff_size(&ids[n], &ids[i]) {
                0 | 1 => {
                    println!("Common chars: {}", common_chars(&ids[n], &ids[i]));
                    return;
                }
                _ => {
                    n = n + 1;
                    continue 'outer;
                }
            }
        }
    }
}

fn diff_size(id1: &str, id2: &str) -> usize {
    id1.chars().zip(id2.chars()).filter(|(a, b)| a != b).count()
}

fn common_chars(id1: &str, id2: &str) -> String {
    id1.chars()
        .zip(id2.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect()
}
