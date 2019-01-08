use std::char;
use std::error;
use std::fmt;
use std::io;
use std::num;
use std::string;

#[derive(Debug)]
pub enum AocError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseChar(char::ParseCharError),
    InvalidToken(string::String),
    InvalidDayProblem,
    ComputeNotFound,
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AocError::Io(ref err) => write!(f, "IO error: {}", err),
            AocError::ParseInt(ref err) => write!(f, "Parse error: {}", err),
            AocError::ParseChar(ref err) => write!(f, "Parse error: {}", err),
            AocError::InvalidToken(ref s) => write!(f, "Parse error: failed to parse string {}", s),
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
            AocError::ParseChar(ref err) => error::Error::description(err),
            AocError::InvalidToken(ref _s) => "Parse error: failed to parse string",
            AocError::InvalidDayProblem => "Error: invalid day and/or problem",
            AocError::ComputeNotFound => "Computation error: no answer found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AocError::Io(ref err) => Some(err),
            AocError::ParseInt(ref err) => Some(err),
            AocError::ParseChar(ref err) => Some(err),
            AocError::InvalidToken(ref _s) => None,
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

impl From<char::ParseCharError> for AocError {
    fn from(err: char::ParseCharError) -> AocError {
        AocError::ParseChar(err)
    }
}

impl From<string::String> for AocError {
    fn from(err: string::String) -> AocError {
        AocError::InvalidToken(err)
    }
}
