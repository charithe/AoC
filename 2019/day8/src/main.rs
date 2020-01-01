use std::fs::File;
use std::io;
use std::io::Read;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

fn main() -> io::Result<()> {
    let decoder = Decoder::new("input")?;
    let mut min_num_zeroes = std::i32::MAX;
    let mut checksum = 0;
    let mut image = [2; SIZE];

    for layer in decoder {
        let (zeroes, ones, twos) = layer
            .iter()
            .enumerate()
            .fold((0, 0, 0), |acc, (idx, byte)| match byte {
                b'0' => {
                    if image[idx] == 2 {
                        image[idx] = 0;
                    }
                    (acc.0 + 1, acc.1, acc.2)
                }
                b'1' => {
                    if image[idx] == 2 {
                        image[idx] = 1;
                    }
                    (acc.0, acc.1 + 1, acc.2)
                }
                b'2' => (acc.0, acc.1, acc.2 + 1),
                _ => unreachable!(),
            });

        if zeroes < min_num_zeroes {
            min_num_zeroes = zeroes;
            checksum = ones * twos;
        }
    }

    println!("Checksum: {}", checksum);

    for i in 0..SIZE {
        if i % WIDTH == 0 {
            print!("\n");
        }

        match image[i] {
            0 => print!(" "),
            1 => print!("1"),
            _ => unreachable!(),
        }
    }

    Ok(())
}

struct Decoder {
    data: File,
}

impl Decoder {
    fn new(path: &str) -> io::Result<Decoder> {
        let data = File::open(path)?;
        Ok(Decoder { data })
    }
}

impl Iterator for Decoder {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; SIZE];
        let result = self.data.read(&mut buf[..]);
        match result {
            Ok(n) if n == SIZE => Some(buf[..n].to_vec()),
            _ => None,
        }
    }
}
