use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use utf8;

#[derive(Debug, Default)]
struct Polymer {
    units: Vec<char>,
    distinct_units: HashSet<char>,
}

impl Polymer {
    fn load<T: Read>(input: T) -> Result<Self> {
        let mut buffered = utf8::BufReadDecoder::new(BufReader::new(input));
        let mut units: Vec<char> = Vec::new();
        let mut distinct_units: HashSet<char> = HashSet::new();

        while let Some(Ok(curr_batch)) = buffered.next_strict() {
            for curr_unit in curr_batch.chars() {
                if curr_unit.is_alphabetic() {
                    units.push(curr_unit);
                    distinct_units.insert(curr_unit.to_ascii_lowercase());
                }
            }
        }

        Ok(Polymer {
            units,
            distinct_units,
        })
    }

    fn react(&self, ignore_unit: Option<char>) -> usize {
        let mut container: Vec<char> = Vec::new();
        for unit in &self.units {
            if ignore_unit
                .filter(|u| u.eq_ignore_ascii_case(unit))
                .is_some()
            {
                continue;
            }

            if let Some(curr_unit) = container.pop() {
                if curr_unit.eq_ignore_ascii_case(unit) && curr_unit != *unit {
                    continue;
                }

                container.push(curr_unit);
            }
            container.push(*unit);
        }

        container.len()
    }

    fn shortest(&self) -> usize {
        self.distinct_units
            .iter()
            .map(|u| self.react(Some(*u)))
            .min()
            .unwrap()
    }
}

fn main() -> Result<()> {
    let input = File::open("input")?;
    let polymer = Polymer::load(input)?;
    println!("Length after reaction: {}", polymer.react(None));
    println!("Shortest possible length: {}", polymer.shortest());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polymer() -> Result<()> {
        let p = "dabAcCaCBAcCcaDA".as_bytes();
        let polymer = Polymer::load(p)?;

        assert_eq!(10, polymer.react(None));
        Ok(())
    }
}
