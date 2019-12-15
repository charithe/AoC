extern crate nom;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::map_res,
    multi::separated_list,
    sequence::tuple,
    IResult,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> aoc::Result<()> {
    let wires = read_wires()?;
    let (wire1, wire2) = (&wires[0], &wires[1]);

    let mut closest = std::i32::MAX;
    let mut shortest = std::i32::MAX;

    let mut wire1_len = 0;
    for seg1 in &wire1.0 {
        let mut wire2_len = 0;
        for seg2 in &wire2.0 {
            if let Some(point) = seg1.intersection_point(&seg2) {
                let dist = (point.x - 0).abs() + (point.y - 0).abs();
                if dist < closest {
                    closest = dist;
                }

                let length =
                    wire1_len + seg1.length_at(&point) + wire2_len + seg2.length_at(&point);
                if length < shortest {
                    shortest = length;
                }
            }
            wire2_len += seg2.length();
        }
        wire1_len += seg1.length();
    }

    println!("Closest intersection: {}", closest);
    println!("Shortest intersection: {}", shortest);
    Ok(())
}

fn read_wires() -> aoc::Result<Vec<Wire>> {
    let input = File::open("input")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(aoc::parse_line(|line| {
            read_wire(&line)
                .map(|(_, w)| w)
                .map_err(|_| aoc::Error::NomParseError)
        }))
        .collect()
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

#[derive(Debug, PartialEq)]
enum Equation {
    X(i32),
    Y(i32),
}

impl Equation {
    fn intersection_point(&self, other: &Equation) -> Option<Point> {
        match (self, other) {
            (Equation::X(x), Equation::Y(y)) | (Equation::Y(y), Equation::X(x)) => {
                Some(Point { x: *x, y: *y })
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Segment {
    equation: Equation,
    start: Point,
    end: Point,
}

impl Segment {
    fn intersection_point(&self, other: &Segment) -> Option<Point> {
        self.equation
            .intersection_point(&other.equation)
            .filter(|point| !point.is_origin())
            .as_ref()
            .and_then(|point| {
                if self.includes_point(point) && other.includes_point(point) {
                    Some(Point { ..*point })
                } else {
                    None
                }
            })
    }

    fn includes_point(&self, point: &Point) -> bool {
        match self.equation {
            Equation::X(x) => {
                point.x == x
                    && point.y >= std::cmp::min(self.start.y, self.end.y)
                    && point.y <= std::cmp::max(self.start.y, self.end.y)
            }
            Equation::Y(y) => {
                point.y == y
                    && point.x >= std::cmp::min(self.start.x, self.end.x)
                    && point.x <= std::cmp::max(self.start.x, self.end.x)
            }
        }
    }

    fn length(&self) -> i32 {
        match self.equation {
            Equation::X(_) => (self.start.y - self.end.y).abs(),
            Equation::Y(_) => (self.start.x - self.end.x).abs(),
        }
    }

    fn length_at(&self, point: &Point) -> i32 {
        match self.equation {
            Equation::X(_) => (self.start.y - point.y).abs(),
            Equation::Y(_) => (self.start.x - point.x).abs(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Wire(Vec<Segment>);

fn read_direction(input: &str) -> IResult<&str, Direction> {
    let (input, (d, a)) = tuple((one_of("RLUD"), map_res(digit1, to_usize)))(input)?;
    let direction = match d {
        'R' => Direction::Right(a),
        'L' => Direction::Left(a),
        'U' => Direction::Up(a),
        'D' => Direction::Down(a),
        _ => unreachable!(),
    };

    Ok((input, direction))
}

fn read_wire(input: &str) -> IResult<&str, Wire> {
    let (input, directions) = separated_list(tag(","), read_direction)(input)?;
    let segments = directions
        .iter()
        .scan(Point { x: 0, y: 0 }, |state, dir| {
            match dir {
                Direction::Left(amt) => {
                    // Equation: y = currY, End: (-amt, currY)
                    let seg = Segment {
                        equation: Equation::Y(state.y),
                        start: Point { ..*state },
                        end: Point {
                            x: state.x - (*amt as i32),
                            ..*state
                        },
                    };
                    *state = Point { ..seg.end };
                    Some(seg)
                }
                Direction::Right(amt) => {
                    // Equation: y = currY, End: (amt, currY)
                    let seg = Segment {
                        equation: Equation::Y(state.y),
                        start: Point { ..*state },
                        end: Point {
                            x: state.x + (*amt as i32),
                            ..*state
                        },
                    };
                    *state = Point { ..seg.end };
                    Some(seg)
                }
                Direction::Up(amt) => {
                    // Equation: x = currX, End: (currX, amt)
                    let seg = Segment {
                        equation: Equation::X(state.x),
                        start: Point { ..*state },
                        end: Point {
                            y: state.y + (*amt as i32),
                            ..*state
                        },
                    };
                    *state = Point { ..seg.end };
                    Some(seg)
                }
                Direction::Down(amt) => {
                    // Equation: x = currX, End: (currX, -amt)
                    let seg = Segment {
                        equation: Equation::X(state.x),
                        start: Point { ..*state },
                        end: Point {
                            y: state.y - (*amt as i32),
                            ..*state
                        },
                    };
                    *state = Point { ..seg.end };
                    Some(seg)
                }
            }
        })
        .collect();

    Ok((input, Wire(segments)))
}

fn to_usize(input: &str) -> Result<usize, std::num::ParseIntError> {
    input.parse::<usize>()
}

#[test]
fn test_read_direction() {
    assert_eq!(read_direction("R345"), Ok(("", Direction::Right(345))));
}

#[test]
fn test_read_wire() {
    assert_eq!(
        read_wire("R1,L2,U3,D4"),
        Ok((
            "",
            Wire(vec![
                Direction::Right(1),
                Direction::Left(2),
                Direction::Up(3),
                Direction::Down(4)
            ])
        ))
    );
}
