use std::error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum AdventOfCodeError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl fmt::Display for AdventOfCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdventOfCodeError::Io(ref err) => write!(f, "IO error: {}", err),
            AdventOfCodeError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for AdventOfCodeError {
    fn description(&self) -> &str {
        match *self {
            AdventOfCodeError::Io(ref err) => err.description(),
            AdventOfCodeError::Parse(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AdventOfCodeError::Io(ref err) => Some(err),
            AdventOfCodeError::Parse(ref err) => Some(err),
        }
    }
}
