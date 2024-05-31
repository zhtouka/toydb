use serde_derive::{Deserialize, Serialize};

/// Result returning Error.
pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(error)
    }
}

/// toyDB errors. All except Internal are considered user-facing.
///
/// TODO: simplify these. Add an IO kind that is used to signal Raft application
/// failure.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Error {
    Abort,
    Assert(String), // TODO include backtrace
    Config(String), // TODO replace with Input
    /// Invalid data, typically decoding errors.
    InvalidData(String),
    Internal(String), // TODO remove?
    Parse(String),    // TODO replace with Input
    ReadOnly,
    Serialization,
    Value(String), // TODO replace with Input or Data
}

/// Constructs an Error::Assert via format!() and into().
#[macro_export]
macro_rules! errassert {
    ($($args:tt)*) => { $crate::error::Error::Assert(format!($($args)*)).into() };
}

/// Constructs an Error::InvalidData via format!() and into().
#[macro_export]
macro_rules! errdata {
    ($($args:tt)*) => { $crate::error::Error::InvalidData(format!($($args)*)).into() };
}

/// Returns an Error::Assert if the given condition is false.
#[macro_export]
macro_rules! asserterr {
    ($cond:expr, $($args:tt)*) => {
        if !$cond { return errassert!($($args)*) }
    };
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Config(s)
            | Error::InvalidData(s)
            | Error::Internal(s)
            | Error::Parse(s)
            | Error::Value(s) => {
                write!(f, "{}", s)
            }
            Error::Assert(s) => write!(f, "assertion failed: {s}"),
            Error::Abort => write!(f, "Operation aborted"),
            Error::Serialization => write!(f, "Serialization failure, retry transaction"),
            Error::ReadOnly => write!(f, "Read-only transaction"),
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::InvalidData(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::InvalidData(msg.to_string())
    }
}

impl From<Box<bincode::ErrorKind>> for Error {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        Error::InvalidData(err.to_string())
    }
}

impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Self {
        Error::Config(err.to_string())
    }
}

impl From<crossbeam::channel::RecvError> for Error {
    fn from(err: crossbeam::channel::RecvError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl<T> From<crossbeam::channel::SendError<T>> for Error {
    fn from(err: crossbeam::channel::SendError<T>) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<crossbeam::channel::TryRecvError> for Error {
    fn from(err: crossbeam::channel::TryRecvError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl<T> From<crossbeam::channel::TrySendError<T>> for Error {
    fn from(err: crossbeam::channel::TrySendError<T>) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<hdrhistogram::CreationError> for Error {
    fn from(err: hdrhistogram::CreationError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<hdrhistogram::RecordError> for Error {
    fn from(err: hdrhistogram::RecordError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<log::ParseLevelError> for Error {
    fn from(err: log::ParseLevelError) -> Self {
        Error::Config(err.to_string())
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error::Config(err.to_string())
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Value(err.to_string())
    }
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(err: rustyline::error::ReadlineError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Error::Value(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::Parse(err.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parse(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Error::Internal(err.to_string())
    }
}
