use std::error;
use std::fmt;
use std::io;
use std::num;
use std::string;

#[derive(Debug)]
pub enum AocError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseString(string::ParseError),
    InvalidDayProblem,
    ComputeNotFound,
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AocError::Io(ref err) => write!(f, "IO error: {}", err),
            AocError::ParseInt(ref err) => write!(f, "Parse error: {}", err),
            AocError::ParseString(ref err) => write!(f, "Parse error: {}", err),
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
            AocError::ParseString(ref err) => error::Error::description(err),
            AocError::InvalidDayProblem => "Error: invalid day and/or problem",
            AocError::ComputeNotFound => "Computation error: no answer found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AocError::Io(ref err) => Some(err),
            AocError::ParseInt(ref err) => Some(err),
            AocError::ParseString(ref err) => Some(err),
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

impl From<string::ParseError> for AocError {
    fn from(err: string::ParseError) -> AocError {
        AocError::ParseString(err)
    }
}
