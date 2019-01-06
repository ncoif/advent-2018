use std::error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum AocError {
    Io(io::Error),
    Parse(num::ParseIntError),
    ComputeNotFound,
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AocError::Io(ref err) => write!(f, "IO error: {}", err),
            AocError::Parse(ref err) => write!(f, "Parse error: {}", err),
            AocError::ComputeNotFound => write!(f, "Computation error: no answer found"),
        }
    }
}

impl error::Error for AocError {
    fn description(&self) -> &str {
        match *self {
            AocError::Io(ref err) => err.description(),
            AocError::Parse(ref err) => error::Error::description(err),
            AocError::ComputeNotFound => "Computation error: no answer found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AocError::Io(ref err) => Some(err),
            AocError::Parse(ref err) => Some(err),
            AocError::ComputeNotFound => None,
        }
    }
}

impl From<io::Error> for AocError {
    fn from(err: io::Error) -> AocError {
        AocError::Io(err)
    }
}

impl From<num::ParseIntError> for AocError {
    fn from(err: num::ParseIntError) -> AocError {
        AocError::Parse(err)
    }
}
