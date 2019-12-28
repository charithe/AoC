use crate::computer;
use std::sync::mpsc;

pub struct AmplifierGroup {
    amps: Vec<Amplifier>,
}

impl AmplifierGroup {
    pub fn new(initial_state: &Vec<i32>, count: usize) -> AmplifierGroup {
        let mut amps = Vec::new();
        for _i in 0..count {
            amps.push(Amplifier::new(initial_state));
        }

        AmplifierGroup { amps }
    }

    pub fn execute(&mut self, phases: &Vec<i32>) -> i32 {
        let mut signal = 0;
        for i in 0..self.amps.len() {
            let amp = &mut self.amps[i];
            signal = amp.run(phases[i], signal);
        }

        signal
    }
}

#[test]
fn test_amplifier_group1() {
    let initial_state = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    let phases = vec![4, 3, 2, 1, 0];
    let mut amps = AmplifierGroup::new(&initial_state, 5);
    let output = amps.execute(&phases);

    assert_eq!(output, 43210);
}

#[test]
fn test_amplifier_group2() {
    let initial_state = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    let phases = vec![0, 1, 2, 3, 4];
    let mut amps = AmplifierGroup::new(&initial_state, 5);
    let output = amps.execute(&phases);

    assert_eq!(output, 54321);
}

#[test]
fn test_amplifier_group3() {
    let initial_state = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    let phases = vec![1, 0, 4, 3, 2];
    let mut amps = AmplifierGroup::new(&initial_state, 5);
    let output = amps.execute(&phases);

    assert_eq!(output, 65210);
}

struct Amplifier {
    comp: computer::Computer,
    input_chan: mpsc::Sender<i32>,
    output_chan: mpsc::Receiver<i32>,
}

impl Amplifier {
    pub fn new(initial_state: &Vec<i32>) -> Amplifier {
        let (input_tx, input_rx) = mpsc::channel();
        let (comp, output_rx) = computer::new(initial_state, input_rx);

        Amplifier {
            comp,
            input_chan: input_tx,
            output_chan: output_rx,
        }
    }

    fn run(&mut self, phase: i32, input_signal: i32) -> i32 {
        self.comp.reset();
        self.input_chan.send(phase).unwrap();
        self.input_chan.send(input_signal).unwrap();
        self.comp.run();
        self.output_chan.recv().unwrap()
    }
}
