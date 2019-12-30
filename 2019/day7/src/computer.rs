use aoc;
use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct IO {
    input_rx: Receiver<i32>,
    output_tx: Sender<i32>,
}

impl IO {
    fn read(&mut self) -> i32 {
        self.input_rx.recv().unwrap()
    }

    fn write(&mut self, val: i32) {
        self.output_tx.send(val).unwrap()
    }
}

#[derive(Debug)]
pub struct Computer {
    initial_state: Vec<i32>,
    memory: Vec<i32>,
    ptr: usize,
    io: IO,
}

pub fn load_input(input_file: &str) -> aoc::Result<Vec<i32>> {
    let file = File::open(input_file)?;
    let buffered = BufReader::new(file);

    buffered
        .split(b',')
        .map(|val| match val {
            Ok(bytes) => aoc::to_i32(bytes),
            Err(err) => Err(aoc::Error::IOError(err)),
        })
        .collect()
}

pub fn new(initial_state: &Vec<i32>, input_rx: Receiver<i32>) -> (Computer, Receiver<i32>) {
    let (output_tx, output_rx) = unbounded();

    (
        Computer {
            initial_state: initial_state.clone(),
            memory: initial_state.clone(),
            ptr: 0,
            io: IO {
                input_rx,
                output_tx,
            },
        },
        output_rx,
    )
}

impl Computer {
    pub fn run(&mut self) -> i32 {
        loop {
            let op = self.next_op();
            for action in op.execute(&mut self.io) {
                match action {
                    Action::Write(dest, val) => self.memory[dest] = val,
                    Action::MoveRel(slots) => self.ptr += slots,
                    Action::MoveAbs(dest) => self.ptr = dest,
                    Action::Halt => {
                        return self.memory[0];
                    }
                }
            }
        }
    }

    fn next_op(&mut self) -> Op {
        let instruction = self.memory[self.ptr];
        let op_code = instruction % 100;
        let modes = ParamModes::from(instruction / 100);

        match op_code {
            1 => {
                let val1 = self.get_param(1, modes.get(1));
                let val2 = self.get_param(2, modes.get(2));
                let dest = self.get_dest_param(3);
                Op::Add { val1, val2, dest }
            }
            2 => {
                let val1 = self.get_param(1, modes.get(1));
                let val2 = self.get_param(2, modes.get(2));
                let dest = self.get_dest_param(3);
                Op::Mul { val1, val2, dest }
            }
            3 => Op::Inp {
                dest: self.get_dest_param(1),
            },
            4 => Op::Out {
                val: self.get_param(1, modes.get(1)),
            },
            5 => {
                let val = self.get_param(1, modes.get(1));
                let loc = self.get_param(2, modes.get(2)) as usize;
                Op::JumpTrue { val, loc }
            }
            6 => {
                let val = self.get_param(1, modes.get(1));
                let loc = self.get_param(2, modes.get(2)) as usize;
                Op::JumpFalse { val, loc }
            }
            7 => {
                let val1 = self.get_param(1, modes.get(1));
                let val2 = self.get_param(2, modes.get(2));
                let dest = self.get_dest_param(3);
                Op::LessThan { val1, val2, dest }
            }
            8 => {
                let val1 = self.get_param(1, modes.get(1));
                let val2 = self.get_param(2, modes.get(2));
                let dest = self.get_dest_param(3);
                Op::Equals { val1, val2, dest }
            }
            99 => Op::Halt,
            _ => unreachable!(),
        }
    }

    fn get_param(&self, num: usize, mode: Mode) -> i32 {
        let p = self.memory[self.ptr + num];
        match mode {
            Mode::Immediate => p,
            Mode::Position => self.memory[p as usize],
        }
    }

    fn get_dest_param(&self, num: usize) -> usize {
        self.memory[self.ptr + num] as usize
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}

struct ParamModes {
    modes: Vec<Mode>,
}

impl ParamModes {
    fn from(code: i32) -> ParamModes {
        let mut modes: Vec<Mode> = Vec::new();

        let mut divisor = 1;
        while code >= divisor * 10 {
            divisor *= 10;
        }

        let mut temp = code;
        while divisor > 0 {
            let d = temp / divisor;
            temp %= divisor;
            divisor /= 10;
            match d {
                0 => modes.push(Mode::Position),
                1 => modes.push(Mode::Immediate),
                _ => unreachable!(),
            }
        }

        modes.reverse();
        ParamModes { modes }
    }

    fn get(&self, param_num: usize) -> Mode {
        self.modes
            .get(param_num - 1)
            .map(|m| *m)
            .unwrap_or(Mode::Position)
    }
}

#[derive(Debug)]
enum Op {
    Add { val1: i32, val2: i32, dest: usize },
    Mul { val1: i32, val2: i32, dest: usize },
    Inp { dest: usize },
    Out { val: i32 },
    JumpTrue { val: i32, loc: usize },
    JumpFalse { val: i32, loc: usize },
    LessThan { val1: i32, val2: i32, dest: usize },
    Equals { val1: i32, val2: i32, dest: usize },
    Halt,
}

impl Op {
    fn execute(&self, io: &mut IO) -> Vec<Action> {
        match self {
            Op::Add { val1, val2, dest } => Op::add(*val1, *val2, *dest),
            Op::Mul { val1, val2, dest } => Op::mul(*val1, *val2, *dest),
            Op::Inp { dest } => Op::inp(io, *dest),
            Op::Out { val } => Op::out(io, *val),
            Op::JumpTrue { val, loc } => Op::jump_true(*val, *loc),
            Op::JumpFalse { val, loc } => Op::jump_false(*val, *loc),
            Op::LessThan { val1, val2, dest } => Op::less_than(*val1, *val2, *dest),
            Op::Equals { val1, val2, dest } => Op::equals(*val1, *val2, *dest),
            Op::Halt => vec![Action::Halt],
        }
    }

    fn add(val1: i32, val2: i32, dest: usize) -> Vec<Action> {
        vec![Action::Write(dest, val1 + val2), Action::MoveRel(4)]
    }

    fn mul(val1: i32, val2: i32, dest: usize) -> Vec<Action> {
        vec![Action::Write(dest, val1 * val2), Action::MoveRel(4)]
    }

    fn inp(io: &mut IO, dest: usize) -> Vec<Action> {
        let val = io.read();
        vec![Action::Write(dest, val), Action::MoveRel(2)]
    }

    fn out(io: &mut IO, val: i32) -> Vec<Action> {
        io.write(val);
        vec![Action::MoveRel(2)]
    }

    fn jump_true(val: i32, loc: usize) -> Vec<Action> {
        if val != 0 {
            vec![Action::MoveAbs(loc)]
        } else {
            vec![Action::MoveRel(3)]
        }
    }

    fn jump_false(val: i32, loc: usize) -> Vec<Action> {
        if val == 0 {
            vec![Action::MoveAbs(loc)]
        } else {
            vec![Action::MoveRel(3)]
        }
    }

    fn less_than(val1: i32, val2: i32, dest: usize) -> Vec<Action> {
        if val1 < val2 {
            vec![Action::Write(dest, 1), Action::MoveRel(4)]
        } else {
            vec![Action::Write(dest, 0), Action::MoveRel(4)]
        }
    }

    fn equals(val1: i32, val2: i32, dest: usize) -> Vec<Action> {
        if val1 == val2 {
            vec![Action::Write(dest, 1), Action::MoveRel(4)]
        } else {
            vec![Action::Write(dest, 0), Action::MoveRel(4)]
        }
    }
}

#[derive(Debug)]
enum Action {
    Write(usize, i32),
    MoveAbs(usize),
    MoveRel(usize),
    Halt,
}
