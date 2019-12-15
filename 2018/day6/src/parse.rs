use nom::types::CompleteStr;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

fn to_uint<T>(input: CompleteStr) -> Result<T, T::Err>
where
    T: FromStr,
{
    input.parse::<T>()
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(num_u32<CompleteStr, u32>, map_res!(take_while!(is_digit), to_uint::<u32>));

named!(parse_coord<CompleteStr,Coord>,
       ws!(do_parse!(x: num_u32 >> tag!(",") >> y: num_u32 >> (Coord{ x, y }))));

pub fn coord(line: &str) -> Option<Coord> {
    parse_coord(CompleteStr(line)).ok().map(|x| x.1)
}
