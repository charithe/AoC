#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

mod cloth;
mod parse;

fn read_claims() -> Result<Vec<parse::Claim>> {
    let input = File::open("input")?;
    let buffered = BufReader::new(input);

    let claims_list = buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| parse::claim(&line))
        .collect();

    Ok(claims_list)
}

fn main() -> Result<()> {
    let claims = read_claims()?;
    let mut c = cloth::Cloth::of_size(1000, 1000);
    claims
        .iter()
        .for_each(|x| c.add_claim(x.left, x.top, x.width, x.height));

    println!("Overlaps: {}", c.find_overlaps());

    let non_overlapping_claim = claims
        .iter()
        .find(|claim| c.has_overlapped(claim.left, claim.top, claim.width, claim.height));

    if let Some(noc) = non_overlapping_claim {
        println!("Non overlapping claim: {}", noc.id)
    }
    Ok(())
}
