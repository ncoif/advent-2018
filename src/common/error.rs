use std::error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum AocError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseString,
    InvalidDayProblem,
    ComputeNotFound,
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AocError::Io(ref err) => write!(f, "IO error: {}", err),
            AocError::ParseInt(ref err) => write!(f, "Parse error: {}", err),
            AocError::ParseString => write!(f, "Parse error: failed to parse string"),
            AocError::InvalidDayProblem => write!(f, "Error: invalid day and/or problem"),
            AocError::ComputeNotFound => write!(f, "Computation error: no answer found"),
        }
    }
}

impl error::Error for AocError {
    fn description(&self) -> &str {
        match *self {
            AocError::Io(ref err) => err.description(),
            AocError::ParseInt(ref err) => error::Error::description(err),
            AocError::ParseString => "Parse error: failed to parse string",
            AocError::InvalidDayProblem => "Error: invalid day and/or problem",
            AocError::ComputeNotFound => "Computation error: no answer found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AocError::Io(ref err) => Some(err),
            AocError::ParseInt(ref err) => Some(err),
            AocError::ParseString => None,
            AocError::InvalidDayProblem => None,
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
        AocError::ParseInt(err)
    }
}
