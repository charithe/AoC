use crate::computer;
use crossbeam::crossbeam_channel::{unbounded, Receiver};
use std::thread;

pub struct AmplifierGroup {
    initial_state: Vec<i32>,
}

impl AmplifierGroup {
    pub fn new(initial_state: &Vec<i32>) -> AmplifierGroup {
        AmplifierGroup {
            initial_state: initial_state.clone(),
        }
    }

    pub fn execute(&mut self, phases: &Vec<i32>) -> i32 {
        let (input_tx0, input_rx0) = unbounded();

        let output_rx0 = self.mk_computer(vec![phases[0], 0], input_rx0);
        let output_rx1 = self.mk_computer(vec![phases[1]], output_rx0);
        let output_rx2 = self.mk_computer(vec![phases[2]], output_rx1);
        let output_rx3 = self.mk_computer(vec![phases[3]], output_rx2);
        let output_rx4 = self.mk_computer(vec![phases[4]], output_rx3);

        let mut signal = 0;
        for s in output_rx4 {
            signal = s;
            input_tx0.send(s).unwrap();
        }

        signal
    }

    fn mk_computer(&self, initial_inputs: Vec<i32>, input_rx: Receiver<i32>) -> Receiver<i32> {
        let (tx, rx) = unbounded();

        let (mut comp, output_rx) = computer::new(&self.initial_state, rx.clone());
        thread::spawn(move || {
            comp.run();
        });

        thread::spawn(move || {
            for inp in initial_inputs.iter() {
                tx.send(*inp).unwrap();
            }

            for signal in input_rx {
                match tx.send(signal) {
                    Ok(_) => (),
                    Err(err) => println!("Error sending {:?}", err.into_inner()),
                }
            }
            drop(tx);
            drop(rx);
        });

        output_rx
    }
}

#[test]
fn test_amplifier_group1() {
    let initial_state = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    let phases = vec![4, 3, 2, 1, 0];
    let mut amps = AmplifierGroup::new(&initial_state);
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
    let mut amps = AmplifierGroup::new(&initial_state);
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
    let mut amps = AmplifierGroup::new(&initial_state);
    let output = amps.execute(&phases);

    assert_eq!(output, 65210);
}

#[test]
fn test_feedback_amplifier_group1() {
    let initial_state = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    let phases = vec![9, 8, 7, 6, 5];
    let mut amps = AmplifierGroup::new(&initial_state);
    let output = amps.execute(&phases);

    assert_eq!(output, 139629729);
}

#[test]
fn test_feedback_amplifier_group2() {
    let initial_state = vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    let phases = vec![9, 7, 8, 5, 6];
    let mut amps = AmplifierGroup::new(&initial_state);
    let output = amps.execute(&phases);

    assert_eq!(output, 18216);
}
