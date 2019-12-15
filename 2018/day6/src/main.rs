#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

mod parse;

fn read_coords_sorted() -> Result<Vec<parse::Coord>> {
    let input = File::open("input")?;
    let buffered = BufReader::new(input);

    let mut coords: Vec<parse::Coord> = buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| parse::coord(&line))
        .collect();

    coords.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    Ok(coords)
}

fn main() -> Result<()> {
    let coords = read_coords_sorted()?;
    println!("{:#?}", coords);
    Ok(())
}
