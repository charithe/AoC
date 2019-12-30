mod amplifier;
mod computer;

fn main() -> aoc::Result<()> {
    let initial_state = computer::load_input("input")?;

    let (s, p) = largest(&initial_state, vec![0, 1, 2, 3, 4]);
    println!("Part1: {:?} => {}", p, s);

    let (s, p) = largest(&initial_state, vec![5, 6, 7, 8, 9]);
    println!("Part2: {:?} => {}", p, s);

    Ok(())
}

fn largest(initial_state: &Vec<i32>, phase_range: Vec<i32>) -> (i32, Vec<i32>) {
    let mut amp_group = amplifier::AmplifierGroup::new(&initial_state);
    let phase_settings = PhaseSettings::new(phase_range);

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

    let (s, p) = largest(&initial_state, vec![0, 1, 2, 3, 4]);
    assert_eq!(s, 54321);
    assert_eq!(p, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_largest2() {
    let initial_state = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];

    let (s, p) = largest(&initial_state, vec![0, 1, 2, 3, 4]);
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

    let (s, p) = largest(&initial_state, vec![0, 1, 2, 3, 4]);
    println!("{:?} => {}", p, s);
    assert_eq!(s, 65210);
    assert_eq!(p, vec![1, 0, 4, 3, 2]);
}

#[test]
fn test_feedback_loop1() {
    let initial_state = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];

    let (s, p) = largest(&initial_state, vec![5, 6, 7, 8, 9]);
    println!("{:?} => {}", p, s);
    assert_eq!(s, 139629729);
    assert_eq!(p, vec![9, 8, 7, 6, 5]);
}

#[test]
fn test_feedback_loop2() {
    let initial_state = vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];

    let (s, p) = largest(&initial_state, vec![5, 6, 7, 8, 9]);
    println!("{:?} => {}", p, s);
    assert_eq!(s, 18216);
    assert_eq!(p, vec![9, 7, 8, 5, 6]);
}
