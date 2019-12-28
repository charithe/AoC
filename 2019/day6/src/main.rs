use aoc;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> aoc::Result<()> {
    let orbits = read_orbits()?;
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for orb in orbits {
        let children = graph.entry(orb.0).or_insert(HashSet::new());
        children.insert(orb.1);
    }

    let count = count_orbits(&graph)?;
    println!("Total orbits: {}", count);

    let transfers = calculate_orbital_transfers(&graph);
    println!("Transfers: {}", transfers);

    Ok(())
}

fn read_orbits() -> aoc::Result<Vec<Orbit>> {
    let input = File::open("input")?;
    let buffered = BufReader::new(input);

    buffered.lines().map(parse_orbit).collect()
}

#[derive(Debug)]
struct Orbit(String, String);

fn parse_orbit(input: std::io::Result<String>) -> aoc::Result<Orbit> {
    let input = input?;
    let mut bodies: Vec<String> = input.split(')').map(|s| s.to_owned()).collect();
    let b1 = bodies.remove(0);
    let b2 = bodies.remove(0);

    Ok(Orbit(b1, b2))
}

fn count_orbits(graph: &HashMap<String, HashSet<String>>) -> aoc::Result<i32> {
    let mut orbit_lengths: HashMap<String, i32> = HashMap::new();
    calculate_orbit_length(graph, &mut orbit_lengths, "COM");

    Ok(orbit_lengths.values().sum())
}

fn calculate_orbit_length(
    graph: &HashMap<String, HashSet<String>>,
    orbit_lengths: &mut HashMap<String, i32>,
    body: &str,
) -> i32 {
    if orbit_lengths.contains_key(body) {
        return orbit_lengths.get(body).map(|v| *v).unwrap();
    }

    let mut length = 0;
    if let Some(children) = graph.get(body) {
        for child in children.iter() {
            let orbit_len = calculate_orbit_length(graph, orbit_lengths, child);
            length += orbit_len + 1;
        }

        orbit_lengths.insert(body.to_string(), length);
    }

    length
}

fn calculate_orbital_transfers(graph: &HashMap<String, HashSet<String>>) -> usize {
    let mut path_to_san: Vec<String> = Vec::new();
    path_to_body(&mut path_to_san, graph, "SAN", "COM");

    let mut path_to_you: Vec<String> = Vec::new();
    path_to_body(&mut path_to_you, graph, "YOU", "COM");

    let mut san_idx = path_to_san.len();
    let mut you_idx = path_to_you.len();

    while path_to_san[san_idx - 1] == path_to_you[you_idx - 1] {
        san_idx -= 1;
        you_idx -= 1;
    }

    path_to_san[0..san_idx].len() + path_to_you[0..you_idx].len()
}

fn path_to_body(
    path: &mut Vec<String>,
    graph: &HashMap<String, HashSet<String>>,
    body: &str,
    curr: &str,
) -> bool {
    if curr == body {
        return true;
    }

    if let Some(children) = graph.get(curr) {
        for child in children.iter() {
            if path_to_body(path, graph, body, child) {
                path.push(curr.to_string());
                return true;
            }
        }
    }

    false
}
