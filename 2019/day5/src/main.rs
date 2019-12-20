mod computer;

fn main() -> aoc::Result<()> {
    let mut comp = computer::load("input")?;
    println!("Result: {}", comp.run());
    Ok(())
}
