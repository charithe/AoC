use aoc;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> aoc::Result<()> {
    let mut computer = Computer::new("input")?;

    computer.reset(12, 2);
    let result = computer.run()?;
    println!("Part1: {}", result);

    for noun in 0..100 {
        for verb in 0..100 {
            computer.reset(noun, verb);
            let result = computer.run()?;
            if result == 19690720 {
                println!(
                    "Part2: Noun={} Verb={} Answer={}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return Ok(());
            }
        }
    }

    Ok(())
}

struct OpArgs {
    arg1: i32,
    arg2: i32,
    pos: usize,
}

#[derive(Debug)]
pub struct Computer {
    input: Vec<i32>,
    memory: Vec<i32>,
    ptr: usize,
}

impl Computer {
    pub fn new(input_file: &str) -> aoc::Result<Computer> {
        let mut computer = Computer {
            input: Vec::new(),
            memory: Vec::new(),
            ptr: 0,
        };

        let file = File::open(input_file)?;
        let buffered = BufReader::new(file);
        let mut buf: Vec<u8> = Vec::new();

        for byte in buffered.bytes() {
            match byte {
                Ok(b',') => {
                    let i = aoc::to_i32(buf.clone())?;
                    computer.input.push(i);
                    buf.clear();
                }
                Ok(b @ b'0'..=b'9') => buf.push(b),
                Ok(_) => (),
                Err(err) => return Err(aoc::Error::IOError(err)),
            }
        }

        if !buf.is_empty() {
            let i = aoc::to_i32(buf.clone())?;
            computer.input.push(i);
        }

        computer.memory = computer.input.clone();
        Ok(computer)
    }

    pub fn reset(&mut self, noun: i32, verb: i32) {
        self.memory = self.input.clone();
        self.memory[1] = noun;
        self.memory[2] = verb;
        self.ptr = 0;
    }

    pub fn run(&mut self) -> aoc::Result<i32> {
        loop {
            let op_code = self.memory[self.ptr];
            match op_code {
                1 => self.add(),
                2 => self.mul(),
                99 => return Ok(self.memory[0]),
                inp @ _ => return Err(aoc::Error::BadOpCode(inp)),
            }
            self.ptr += 4
        }
    }

    fn add(&mut self) {
        let args = self.read_args();
        self.memory[args.pos] = args.arg1 + args.arg2
    }

    fn mul(&mut self) {
        let args = self.read_args();
        self.memory[args.pos] = args.arg1 * args.arg2
    }

    fn read_args(&mut self) -> OpArgs {
        OpArgs {
            arg1: self.memory[self.input[self.ptr + 1] as usize],
            arg2: self.memory[self.input[self.ptr + 2] as usize],
            pos: self.memory[self.ptr + 3] as usize,
        }
    }
}
