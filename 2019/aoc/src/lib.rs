extern crate nom;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ParseIntError(num::ParseIntError),
    ToStringError(std::string::FromUtf8Error),
    BadOpCode(i32),
    NomParseError,
}

impl StdError for Error {
    fn description(&self) -> &str {
        "description"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IOError(err) => f.write_fmt(format_args!("IO error: {}", err)),
            Error::ParseIntError(err) => f.write_fmt(format_args!("Parse error: {}", err)),
            Error::ToStringError(err) => f.write_fmt(format_args!("ToString error: {}", err)),
            Error::BadOpCode(code) => f.write_fmt(format_args!("Bad op code: {}", code)),
            Error::NomParseError => f.write_str("Parse error"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseIntError(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error::ToStringError(err)
    }
}

impl<T> From<nom::Err<T>> for Error {
    fn from(err: nom::Err<T>) -> Error {
        Error::NomParseError
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse_i32(line: std::io::Result<String>) -> Result<i32> {
    let s = line?;
    let v = s.parse::<i32>()?;
    Ok(v)
}

pub fn to_i32(buf: Vec<u8>) -> Result<i32> {
    let s = String::from_utf8(buf)?;
    let i = s.parse::<i32>()?;
    Ok(i)
}

pub fn parse_line<T, F>(convert: F) -> impl Fn(std::io::Result<String>) -> Result<T>
where
    F: Fn(String) -> Result<T>,
{
    move |line| convert(line?)
}
