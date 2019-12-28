mod amplifier;
mod computer;

fn main() -> aoc::Result<()> {
    let initial_state = computer::load_input("input")?;
    let (s, p) = largest(&initial_state);
    println!("{:?} => {}", p, s);

    Ok(())
}

fn largest(initial_state: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut amp_group = amplifier::AmplifierGroup::new(&initial_state, 5);
    let phase_settings = PhaseSettings::new(vec![0, 1, 2, 3, 4]);

    let mut largest_signal = 0;
    let mut largest_phase: Vec<i32> = Vec::new();
    for phases in phase_settings {
        let signal = amp_group.execute(&phases);
        if signal > largest_signal {
            largest_signal = signal;
            largest_phase = phases;
        }
    }
    (largest_signal, largest_phase)
}

struct PhaseSettings {
    values: Vec<Vec<i32>>,
}

impl PhaseSettings {
    fn new(values: Vec<i32>) -> PhaseSettings {
        let c = Self::combinations(values);
        PhaseSettings { values: c }
    }

    fn combinations(values: Vec<i32>) -> Vec<Vec<i32>> {
        if values.len() <= 1 {
            return vec![values.clone()];
        }

        let mut collection: Vec<Vec<i32>> = Vec::new();

        let mut candidates = values.clone();
        for _i in 0..candidates.len() {
            let head = candidates.remove(0);
            let combinations = Self::combinations(candidates.to_vec());
            for c in combinations {
                let mut x = vec![head];
                x.extend_from_slice(&c);
                collection.push(x);
            }
            candidates.push(head);
        }
        collection
    }
}

impl Iterator for PhaseSettings {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.pop()
    }
}

#[test]
fn test_largest1() {
    let initial_state = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];

    let (s, p) = largest(&initial_state);
    assert_eq!(s, 54321);
    assert_eq!(p, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_largest2() {
    let initial_state = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];

    let (s, p) = largest(&initial_state);
    println!("{:?} => {}", p, s);
    assert_eq!(s, 43210);
    assert_eq!(p, vec![4, 3, 2, 1, 0]);
}

#[test]
fn test_largest3() {
    let initial_state = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];

    let (s, p) = largest(&initial_state);
    println!("{:?} => {}", p, s);
    assert_eq!(s, 65210);
    assert_eq!(p, vec![1, 0, 4, 3, 2]);
}

/*
use crossbeam::crossbeam_channel::bounded;
use std::thread;
fn largest(initial_state: &Vec<i32>) -> (i32, Vec<i32>) {
    let (phase_tx, phase_rx) = bounded(32);
    thread::spawn(move || {
        for a in 0..=4 {
            for b in 0..=4 {
                for c in 0..=4 {
                    for d in 0..=4 {
                        for e in 0..=4 {
                            let phases = vec![a, b, c, d, e];
                            phase_tx.send(phases).unwrap()
                        }
                    }
                }
            }
        }
        drop(phase_tx);
    });

    let initial_state = computer::load_input("input")?;
    let mut amp_group = amplifier::AmplifierGroup::new(&initial_state, 5);
    let mut largest_signal = 0;
    for phase in phase_rx.iter() {
        let signal = amp_group.execute(&phase);
        if signal > largest_signal {
            largest_signal = signal;
        }
    }

    println!("Largest: {}", largest_signal);


        let initial_state = computer::load_input("input")?;
        let (signal_tx, signal_rx) = bounded(32);

        for _i in 0..10 {
            let (phase_recv, signal_send) = (phase_rx.clone(), signal_tx.clone());
            let mut amp_group = amplifier::AmplifierGroup::new(&initial_state, 5);
            thread::spawn(move || {
                for phase in phase_recv.iter() {
                    let signal = amp_group.execute(&phase);
                    signal_send.send((signal, phase)).unwrap();
                }
                drop(signal_send);
            });
        }
        drop(signal_tx);

        let mut largest_signal = 0;
        let mut largest_phase: Vec<i32> = Vec::new();

        for (s, p) in signal_rx.iter() {
            println!("{} {} {}", s, largest_signal, s > largest_signal);
            if s > largest_signal {
                largest_signal = s;
                largest_phase = p.clone();
            }
        }

        println!(
            "Largest phase settings: {} -> {:?}",
            largest_signal, largest_phase
        );
}
*/
