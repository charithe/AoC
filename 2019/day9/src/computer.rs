use aoc;
use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct IO {
    input_rx: Receiver<i64>,
    output_tx: Sender<i64>,
}

impl IO {
    fn read(&mut self) -> i64 {
        self.input_rx.recv().unwrap()
    }

    fn write(&mut self, val: i64) {
        self.output_tx.send(val).unwrap()
    }
}

#[derive(Debug)]
pub struct Computer {
    memory: Vec<i64>,
    ptr: usize,
    relative_base: i64,
    io: IO,
}

pub fn load_input(input_file: &str) -> aoc::Result<Vec<i64>> {
    let file = File::open(input_file)?;
    let buffered = BufReader::new(file);

    buffered
        .split(b',')
        .map(|val| match val {
            Ok(bytes) => aoc::to_i64(bytes),
            Err(err) => Err(aoc::Error::IOError(err)),
        })
        .collect()
}

pub fn new(initial_state: &Vec<i64>, input_rx: Receiver<i64>) -> (Computer, Receiver<i64>) {
    let (output_tx, output_rx) = unbounded();

    (
        Computer {
            memory: initial_state.clone(),
            ptr: 0,
            relative_base: 0,
            io: IO {
                input_rx,
                output_tx,
            },
        },
        output_rx,
    )
}

impl Computer {
    pub fn run(&mut self) -> i64 {
        loop {
            let op = self.next_op();
            for action in op.execute(&mut self.io) {
                match action {
                    Action::Write(dest, val) => {
                        if dest >= self.memory.len() {
                            self.memory.resize(dest + 1, 0);
                        }
                        self.memory[dest] = val
                    }
                    Action::MoveRel(slots) => self.ptr += slots,
                    Action::MoveAbs(dest) => self.ptr = dest,
                    Action::SetRelativeOffset(offset) => self.relative_base += offset,
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
                let dest = self.get_dest_param(3, modes.get(3));
                Op::Add { val1, val2, dest }
            }
            2 => {
                let val1 = self.get_param(1, modes.get(1));
                let val2 = self.get_param(2, modes.get(2));
                let dest = self.get_dest_param(3, modes.get(3));
                Op::Mul { val1, val2, dest }
            }
            3 => Op::Inp {
                dest: self.get_dest_param(1, modes.get(1)),
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
                let dest = self.get_dest_param(3, modes.get(3));
                Op::LessThan { val1, val2, dest }
            }
            8 => {
                let val1 = self.get_param(1, modes.get(1));
                let val2 = self.get_param(2, modes.get(2));
                let dest = self.get_dest_param(3, modes.get(3));
                Op::Equals { val1, val2, dest }
            }
            9 => {
                let offset = self.get_param(1, modes.get(1));
                Op::RelativeBaseOffset { offset }
            }
            99 => Op::Halt,
            _ => unreachable!(),
        }
    }

    fn get_param(&self, num: usize, mode: Mode) -> i64 {
        let p = self.memory[self.ptr + num];
        match mode {
            Mode::Immediate => p,
            Mode::Position => self.memory.get(p as usize).map(|v| *v).unwrap_or(0),
            Mode::Relative => self
                .memory
                .get((p + self.relative_base) as usize)
                .map(|v| *v)
                .unwrap_or(0),
        }
    }

    fn get_dest_param(&self, num: usize, mode: Mode) -> usize {
        let p = self.memory[self.ptr + num];
        match mode {
            Mode::Position => p as usize,
            Mode::Relative => (self.relative_base + p) as usize,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

struct ParamModes {
    modes: Vec<Mode>,
}

impl ParamModes {
    fn from(code: i64) -> ParamModes {
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
                2 => modes.push(Mode::Relative),
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
    Add { val1: i64, val2: i64, dest: usize },
    Mul { val1: i64, val2: i64, dest: usize },
    Inp { dest: usize },
    Out { val: i64 },
    JumpTrue { val: i64, loc: usize },
    JumpFalse { val: i64, loc: usize },
    LessThan { val1: i64, val2: i64, dest: usize },
    Equals { val1: i64, val2: i64, dest: usize },
    RelativeBaseOffset { offset: i64 },
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
            Op::RelativeBaseOffset { offset } => Op::relative_base_offset(*offset),
            Op::Halt => vec![Action::Halt],
        }
    }

    fn add(val1: i64, val2: i64, dest: usize) -> Vec<Action> {
        vec![Action::Write(dest, val1 + val2), Action::MoveRel(4)]
    }

    fn mul(val1: i64, val2: i64, dest: usize) -> Vec<Action> {
        vec![Action::Write(dest, val1 * val2), Action::MoveRel(4)]
    }

    fn inp(io: &mut IO, dest: usize) -> Vec<Action> {
        let val = io.read();
        vec![Action::Write(dest, val), Action::MoveRel(2)]
    }

    fn out(io: &mut IO, val: i64) -> Vec<Action> {
        io.write(val);
        vec![Action::MoveRel(2)]
    }

    fn jump_true(val: i64, loc: usize) -> Vec<Action> {
        if val != 0 {
            vec![Action::MoveAbs(loc)]
        } else {
            vec![Action::MoveRel(3)]
        }
    }

    fn jump_false(val: i64, loc: usize) -> Vec<Action> {
        if val == 0 {
            vec![Action::MoveAbs(loc)]
        } else {
            vec![Action::MoveRel(3)]
        }
    }

    fn less_than(val1: i64, val2: i64, dest: usize) -> Vec<Action> {
        if val1 < val2 {
            vec![Action::Write(dest, 1), Action::MoveRel(4)]
        } else {
            vec![Action::Write(dest, 0), Action::MoveRel(4)]
        }
    }

    fn equals(val1: i64, val2: i64, dest: usize) -> Vec<Action> {
        if val1 == val2 {
            vec![Action::Write(dest, 1), Action::MoveRel(4)]
        } else {
            vec![Action::Write(dest, 0), Action::MoveRel(4)]
        }
    }

    fn relative_base_offset(offset: i64) -> Vec<Action> {
        vec![Action::SetRelativeOffset(offset), Action::MoveRel(2)]
    }
}

#[derive(Debug)]
enum Action {
    Write(usize, i64),
    MoveAbs(usize),
    MoveRel(usize),
    SetRelativeOffset(i64),
    Halt,
}
