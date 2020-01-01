use aoc;
use crossbeam::crossbeam_channel::unbounded;
use std::thread;

mod computer;

fn main() -> aoc::Result<()> {
    let initial_state = computer::load_input("input")?;
    let boost = run_computer(&initial_state, Some(1));
    println!("BOOST key code: {:?}", boost);

    let coords = run_computer(&initial_state, Some(2));
    println!("Coordinates: {:?}", coords);

    Ok(())
}

fn run_computer(initial_state: &Vec<i64>, input: Option<i64>) -> Vec<i64> {
    let (tx, rx) = unbounded();
    let (mut comp, output) = computer::new(initial_state, rx.clone());
    thread::spawn(move || {
        comp.run();
    });

    input.iter().for_each(|v| tx.send(*v).unwrap());
    output.iter().collect()
}

#[test]
fn test_case1() {
    let initial_state = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];

    let output = run_computer(&initial_state, None);
    assert_eq!(initial_state, output);
}

#[test]
fn test_case2() {
    let initial_state = vec![104, 1125899906842624, 99];

    let output = run_computer(&initial_state, None);
    assert_eq!(vec![1125899906842624], output);
}
